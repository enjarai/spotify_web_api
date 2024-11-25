use super::private::AuthFlow;
use crate::{
    api::{ApiError, FormParams},
    model::Token,
    RestError,
};
use base64::{engine::general_purpose, Engine as _};
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

impl ClientCredentials {
    /// Creates a new instance of `ClientCredentials` with the provided client ID and secret.
    ///
    /// This constructor is used to create a `ClientCredentials` instance, which can then be used
    /// to authenticate with Spotify's API by requesting an access token.
    ///
    /// # Parameters
    /// - `client_id`: The Client ID of your Spotify application.
    /// - `client_secret`: The Client Secret of your Spotify application.
    ///
    /// # Returns
    /// A new instance of `ClientCredentials`.
    ///
    /// # Example
    /// ```
    /// use spotify_web_api::auth::ClientCredentials;
    ///
    /// let client_credentials = ClientCredentials::new("your-client-id", "your-client-secret");
    /// ```
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
