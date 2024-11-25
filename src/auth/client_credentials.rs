use super::private::AuthFlow;
use crate::{
    api::{ApiError, FormParams},
    model::Token,
    RestError,
};
use base64::{engine::general_purpose, Engine as _};
use reqwest::blocking::Client;

pub struct ClientCredentials {
    client_id: String,
    client_secret: String,
}

impl AuthFlow for ClientCredentials {}

impl ClientCredentials {
    pub fn new(client_id: impl Into<String>, client_secret: impl Into<String>) -> Self {
        Self {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
        }
    }

    pub(crate) fn request_token(&self, client: &Client) -> Result<Token, ApiError<RestError>> {
        let credentials = format!("{}:{}", self.client_id, self.client_secret);
        let mut auth = general_purpose::URL_SAFE_NO_PAD.encode(credentials);
        auth.insert_str(0, "Basic ");

        let mut params = FormParams::default();
        params.push("grant_type", &"client_credentials");

        super::request_token(client, Some(auth), params)
    }
}
