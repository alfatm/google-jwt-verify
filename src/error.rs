use crate::algorithm::Algorithm;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidToken(&'static str),
    Base64Error(base64::DecodeError),
    SerdeError(String),
    RetrieveKeyFailure,
    UnsupportedAlgorithm(Algorithm),
    Expired,
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
