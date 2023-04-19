#[cfg(test)]
mod test;

mod algorithm;
mod client;
mod error;
mod header;
mod jwk;
mod key_provider;
mod token;
mod unverified_token;

pub use crate::client::Client;
pub use crate::token::{IdPayload, RequiredClaims, Token};
pub use error::Error;

fn base64_decode(input: &str) -> Result<Vec<u8>, base64::DecodeError> {
    use base64::Engine as _;
    base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(&input)
}
