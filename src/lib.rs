#[cfg(test)]
mod test;

mod algorithm;
mod error;
mod header;
mod jwk;
mod key_provider;
mod token;
mod unverified_token;

#[cfg(feature = "blocking")]
mod client_blocking;

#[cfg(feature = "async")]
mod client_async;

#[cfg(feature = "blocking")]
pub use client_blocking::*;

#[cfg(feature = "async")]
pub use client_async::*;

pub use crate::token::{IdPayload, RequiredClaims, Token};
pub use error::Error;

fn base64_decode(input: &str) -> Result<Vec<u8>, base64::DecodeError> {
    use base64::Engine as _;
    base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(input)
}
