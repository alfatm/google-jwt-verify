use crate::error::Error;
use crate::key_provider::KeyProvider;
use crate::token::IdPayload;
use crate::token::Token;
use crate::unverified_token::UnverifiedToken;
use serde::Deserialize;
use std::sync::Arc;
use std::sync::Mutex;

pub struct ClientBlocking<T = GoogleKeyProvider> {
    client_id: String,
    key_provider: Arc<Mutex<T>>,
    check_expiration: bool,
}

impl<KP: Default> ClientBlocking<KP> {
    pub fn new(client_id: &str) -> Self {
        Self {
            client_id: client_id.to_owned(),
            key_provider: Arc::new(Mutex::new(KP::default())),
            check_expiration: true,
        }
    }
}

impl<KP> ClientBlocking<KP> {
    pub fn new_with_provider(client_id: &str, provider: KP) -> Self {
        ClientBlocking {
            client_id: client_id.to_string(),
            key_provider: Arc::new(Mutex::new(provider)),
            check_expiration: true,
        }
    }

    pub fn unsafe_ignore_expiration(mut self) -> Self {
        self.check_expiration = false;
        self
    }
}

impl<KP: KeyProvider> ClientBlocking<KP> {
    pub fn verify_token_with_payload<P>(&self, token_string: &str) -> Result<Token<P>, Error>
    where
        for<'a> P: Deserialize<'a>,
    {
        let unverified_token =
            UnverifiedToken::<P>::validate(token_string, self.check_expiration, &self.client_id)?;
        let mut key_provider = self.key_provider.lock().map_err(|_| Error::MutexPoisoned)?;
        unverified_token.verify(&mut *key_provider)
    }

    pub fn verify_token(&self, token_string: &str) -> Result<Token<()>, Error> {
        self.verify_token_with_payload::<()>(token_string)
    }

    pub fn verify_id_token(&self, token_string: &str) -> Result<Token<IdPayload>, Error> {
        self.verify_token_with_payload(token_string)
    }
}
