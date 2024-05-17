use crate::algorithm::Algorithm;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("Invalid token: {0}")]
    InvalidToken(&'static str),

    #[error("Failed to decode from Base64: {0}")]
    Base64Error(base64::DecodeError),

    #[error("Failed to deserialize data: {0}")]
    SerdeError(String),

    #[error("Failed to retrieve the key")]
    RetrieveKeyFailure,

    #[error("Unsupported algorithm: {0:?}")]
    UnsupportedAlgorithm(Algorithm),

    #[error("JWT token has expired")]
    Expired,

    #[error("Mutex poisoned")]
    MutexPoisoned,
}

impl From<base64::DecodeError> for Error {
    fn from(_: base64::DecodeError) -> Self {
        Error::InvalidToken("decode error")
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::SerdeError(err.to_string())
    }
}

impl From<openssl::error::ErrorStack> for Error {
    fn from(_: openssl::error::ErrorStack) -> Self {
        Error::InvalidToken("openssl error")
    }
}
