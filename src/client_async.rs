use crate::error::Error;
use crate::key_provider::AsyncKeyProvider;
use crate::key_provider::GoogleKeyProvider;
use crate::token::IdPayload;
use crate::token::Token;
use crate::unverified_token::UnverifiedToken;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct ClientAsync<T = GoogleKeyProvider> {
    client_id: String,
    key_provider: Arc<Mutex<T>>,
    check_expiration: bool,
}

impl<KP: Default> ClientAsync<KP> {
    pub fn new(client_id: &str) -> Self {
        Self {
            client_id: client_id.to_owned(),
            key_provider: Arc::new(Mutex::new(KP::default())),
            check_expiration: true,
        }
    }
}

impl<KP> ClientAsync<KP> {
    pub fn new_with_provider(client_id: &str, provider: KP) -> Self {
        Self {
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

impl<KP: AsyncKeyProvider> ClientAsync<KP> {
    pub async fn verify_token_with_payload_async<P>(
        &self,
        token_string: &str,
    ) -> Result<Token<P>, Error>
    where
        for<'a> P: Deserialize<'a> + Send + Sync,
    {
        let unverified_token =
            UnverifiedToken::<P>::validate(token_string, self.check_expiration, &self.client_id)?;

        let mut key_provider = self.key_provider.lock().await;
        unverified_token.verify_async(&mut *key_provider).await
    }

    pub async fn verify_token_async(&self, token_string: &str) -> Result<Token<()>, Error> {
        self.verify_token_with_payload_async::<()>(token_string)
            .await
    }

    pub async fn verify_id_token_async(
        &self,
        token_string: &str,
    ) -> Result<Token<IdPayload>, Error> {
        self.verify_token_with_payload_async(token_string).await
    }
}
