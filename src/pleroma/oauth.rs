use crate::oauth;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppDataFromServer {
    id: String,
    name: String,
    website: Option<String>,
    redirect_uri: String,
    client_id: String,
    client_secret: String,
}

/// Obteined token data from server.
#[derive(Debug, Deserialize, Clone)]
pub struct TokenDataFromServer {
    access_token: String,
    token_type: String,
    scope: String,
    created_at: u64,
    expires_in: Option<u64>,
    refresh_token: Option<String>,
}

impl Into<oauth::AppData> for AppDataFromServer {
    fn into(self) -> oauth::AppData {
        oauth::AppData::new(
            self.id,
            self.name,
            self.website,
            Some(self.redirect_uri),
            self.client_id,
            self.client_secret,
        )
    }
}

impl Into<oauth::TokenData> for TokenDataFromServer {
    fn into(self) -> oauth::TokenData {
        oauth::TokenData::new(
            self.access_token,
            self.token_type,
            Some(self.scope),
            Some(self.created_at),
            self.expires_in,
            self.refresh_token,
        )
    }
}
