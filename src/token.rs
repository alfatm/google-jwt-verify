use serde_derive::Deserialize;

pub struct Token<P> {
    required_claims: RequiredClaims,
    payload: P,
}

impl<P> Token<P> {
    pub fn new(required_claims: RequiredClaims, payload: P) -> Token<P> {
        Token {
            required_claims,
            payload,
        }
    }
    pub fn get_claims(&self) -> RequiredClaims {
        self.required_claims.clone()
    }
    pub fn get_payload(&self) -> &P {
        &self.payload
    }
}

#[derive(Deserialize, Clone)]
pub struct RequiredClaims {
    #[serde(rename = "iss")]
    issuer: String,

    #[serde(rename = "sub")]
    subject: String,

    #[serde(rename = "aud")]
    audience: String,

    #[serde(rename = "azp")]
    android_audience: String,

    #[serde(rename = "iat")]
    issued_at: u64,

    #[serde(rename = "exp")]
    expires_at: u64,
}

impl RequiredClaims {
    pub fn get_issuer(&self) -> String {
        self.issuer.clone()
    }
    pub fn get_subject(&self) -> String {
        self.subject.clone()
    }
    pub fn get_audience(&self) -> String {
        self.audience.clone()
    }
    pub fn get_android_audience(&self) -> String {
        self.android_audience.clone()
    }
    pub fn get_issued_at(&self) -> u64 {
        self.issued_at
    }
    pub fn get_expires_at(&self) -> u64 {
        self.expires_at
    }
}

#[derive(Deserialize, Clone)]
pub struct IdPayload {
    pub email: String,
    pub email_verified: bool,
    pub name: String,
    pub picture: String,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub locale: Option<String>,
    #[serde(rename = "hd")]
    pub domain: Option<String>,
}
