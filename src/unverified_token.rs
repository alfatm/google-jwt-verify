use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(feature = "async")]
use crate::key_provider::AsyncKeyProvider;
#[cfg(feature = "blocking")]
use crate::key_provider::KeyProvider;
use crate::{base64_decode, header::Header, jwk::JsonWebKey, Error, RequiredClaims, Token};

pub struct UnverifiedToken<P> {
    header: Header,
    signed_body: String,
    signature: Vec<u8>,
    claims: RequiredClaims,
    json_payload: P,
}

impl<P> UnverifiedToken<P>
where
    for<'a> P: Deserialize<'a>,
{
    pub fn validate(
        token_string: &str,
        check_expiration: bool,
        client_id: &str,
    ) -> Result<Self, Error> {
        let mut segments = token_string.split('.');
        let encoded_header = segments
            .next()
            .ok_or(Error::InvalidToken("invalid header"))?;
        let encoded_payload = segments
            .next()
            .ok_or(Error::InvalidToken("invalid payload"))?;
        let encoded_signature = segments
            .next()
            .ok_or(Error::InvalidToken("invalid signature"))?;

        let header: Header = serde_json::from_slice(&base64_decode(encoded_header)?)?;
        let signed_body = format!("{}.{}", encoded_header, encoded_payload);
        let signature = base64_decode(encoded_signature)?;
        let payload = base64_decode(encoded_payload)?;
        let claims: RequiredClaims = serde_json::from_slice(&payload)?;
        if claims.get_audience() != client_id {
            return Err(Error::InvalidToken("invalid audience"));
        }
        let issuer = claims.get_issuer();
        if issuer != "https://accounts.google.com" && issuer != "accounts.google.com" {
            return Err(Error::InvalidToken("invalid issuer"));
        }
        let current_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        if check_expiration && claims.get_expires_at() < current_timestamp {
            return Err(Error::Expired);
        }
        if claims.get_issued_at() > claims.get_expires_at() {
            return Err(Error::InvalidToken("invalid expires at"));
        }
        let json_payload: P = serde_json::from_slice(&payload)?;
        Ok(Self {
            claims,
            signature,
            signed_body,
            json_payload,
            header,
        })
    }
}

impl<P> UnverifiedToken<P> {
    #[cfg(feature = "blocking")]
    pub fn verify<KP: KeyProvider>(self, key_provider: &mut KP) -> Result<Token<P>, Error> {
        let key_id = self.header.key_id.clone();
        self.verify_with_key(key_provider.get_key(&key_id))
    }
    #[cfg(feature = "async")]
    pub async fn verify_async<KP: AsyncKeyProvider>(
        self,
        key_provider: &mut KP,
    ) -> Result<Token<P>, Error> {
        let key_id = self.header.key_id.clone();
        self.verify_with_key(key_provider.get_key_async(&key_id).await)
    }
    fn verify_with_key(self, key: Result<Option<JsonWebKey>, ()>) -> Result<Token<P>, Error> {
        let key = match key {
            Ok(Some(key)) => key,
            Ok(None) => return Err(Error::InvalidToken("missing json web key")),
            Err(_) => return Err(Error::RetrieveKeyFailure),
        };
        key.verify(self.signed_body.as_bytes(), &self.signature)?;
        Ok(Token::new(self.claims, self.json_payload))
    }
}
