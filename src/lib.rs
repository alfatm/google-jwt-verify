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
use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
pub use error::Error;

pub const BASE64_ENGINE: engine::GeneralPurpose =
    engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

fn base64_decode(input: &str) -> Result<Vec<u8>, base64::DecodeError> {
    BASE64_ENGINE.decode(&input)
}
