use super::private::{AsyncAuthFlow, AuthFlow};
use crate::{
    RestError,
    api::{ApiError, FormParams},
    model::Token,
};
use base64::{Engine as _, engine::general_purpose};
use reqwest::blocking::Client;

/// Represents the Client Credentials authentication flow for Spotify.
///
/// This struct is used for authenticating with Spotify's API using the Client Credentials flow,
/// which is commonly used for server-to-server requests where user authorization is not required.
/// This flow involves sending a request to the Spotify Accounts service to obtain an access token
/// using the client ID and client secret.
pub struct ClientCredentials {
    /// The Client ID generated after registering your application.
    ///
    /// This is required to identify your application to the Spotify API.
    client_id: String,

    /// The Client Secret generated after registering your application.
    ///
    /// This is required to authenticate your application to the Spotify API.
    client_secret: String,
}

impl AuthFlow for ClientCredentials {}
impl AsyncAuthFlow for ClientCredentials {}

impl ClientCredentials {
    pub fn new(client_id: impl Into<String>, client_secret: impl Into<String>) -> Self {
        Self {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
        }
    }

    pub fn request_token(&self, client: &Client) -> Result<Token, ApiError<RestError>> {
        let (auth, params) = self.auth_value_and_params();
        super::request_token(client, Some(auth), params)
    }

    pub async fn request_token_async(
        &self,
        client: &reqwest::Client,
    ) -> Result<Token, ApiError<RestError>> {
        let (auth, params) = self.auth_value_and_params();
        super::request_token_async(client, Some(auth), params).await
    }

    fn auth_value_and_params(&self) -> (String, FormParams<'_>) {
        let credentials = format!("{}:{}", self.client_id, self.client_secret);
        let mut auth = general_purpose::URL_SAFE_NO_PAD.encode(credentials);
        auth.insert_str(0, "Basic ");

        let mut params = FormParams::default();
        params.push("grant_type", &"client_credentials");

        (auth, params)
    }
}
