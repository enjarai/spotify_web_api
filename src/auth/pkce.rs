use super::{
    AuthError, AuthResult,
    private::{AsyncRefresh, AuthFlow, Refresh},
};
use crate::{
    RestError,
    api::{ApiError, FormParams, QueryParams},
    auth::scopes::{self, Scope},
    model::Token,
};
use async_trait::async_trait;
use reqwest::blocking::Client;
use std::collections::HashSet;
use url::Url;

/// Represents the Authorization Code Flow with Proof Key for Code Exchange (PKCE).
///
/// This flow is used to securely authenticate users and obtain access tokens for the Spotify Web API.
/// It is particularly suited for client-side and mobile applications where the client secret cannot
/// be securely stored.
///
/// For more details, see the [Spotify Authorization Guide](https://developer.spotify.com/documentation/web-api/tutorials/code-pkce-flow).
#[derive(Debug, Clone)]
pub struct AuthCodePKCE {
    /// The Client ID generated after registering your application.
    ///
    /// This is required to identify your application to the Spotify API.
    client_id: String,

    /// The URI to redirect to after the user grants or denies permission.
    ///
    /// This URI must match one of the Redirect URIs you specified when registering your application.
    /// If the redirect URI does not match, the authorization request will fail.
    /// For details on redirect URIs, see the [Spotify App Guide](https://developer.spotify.com/documentation/web-api/concepts/apps).
    redirect_uri: String,

    /// A space-separated list of requested [scopes](https://developer.spotify.com/documentation/web-api/concepts/scopes).
    ///
    /// Scopes determine the level of access your application is requesting from the user.
    /// If no scopes are specified, access will be granted only to publicly available information.
    /// This includes content visible in the Spotify desktop, web, and mobile players.
    scopes: Option<HashSet<Scope>>,

    /// A cryptographically secure random string to be used as the `state` parameter.
    ///
    /// The `state` parameter ensures that the response to the authorization request is not
    /// the result of a CSRF attack. When a user authorization URL is requested, a 16-character random
    /// string is generated.
    state: Option<String>,

    /// A randomly generated string used to verify the integrity of the authorization process.
    ///
    /// The code verifier is a cryptographically random string that your application generates at the
    /// start of the authorization process. It must be between 43 and 128 characters in length and
    /// contain only alphanumeric characters and the following characters: `-`, `_`, `.`, `~`.
    ///
    /// During the authorization request, a SHA-256 hash of this string (the `code_challenge`) is sent to
    /// the Spotify authorization server. During the token request, this original string is sent back to
    /// verify the integrity of the exchange.
    ///
    /// For more details, see [RFC 7636](https://datatracker.ietf.org/doc/html/rfc7636).
    code_verifier: Option<String>,
}

impl AuthCodePKCE {
    /// Creates a new instance of the `AuthCodePKCE` struct.
    ///
    /// This method initializes the struct with the provided client ID, redirect URI, and optional scopes.
    /// It sets the `state` and `code_verifier` fields to `None`, as they will be generated
    /// during the authorization process.
    ///
    /// # Parameters
    /// - `client_id`: The Client ID of your Spotify application.
    /// - `redirect_uri`: The URI to redirect to after user authorization.
    /// - `scopes`: An optional set of scopes defining the level of access requested from the user.
    ///
    /// # Returns
    /// A new instance of `AuthCodePKCE` with the provided values.
    ///
    /// # Example
    /// ```
    /// use std::collections::HashSet;
    /// use spotify_web_api::auth::{AuthCodePKCE, scopes};
    ///
    /// let pkce = AuthCodePKCE::new("client_id", "http://localhost:8080/callback", scopes::user_details());
    /// ```
    pub fn new(
        client_id: impl Into<String>,
        redirect_uri: impl Into<String>,
        scopes: impl Into<Option<HashSet<Scope>>>,
    ) -> Self {
        Self {
            client_id: client_id.into(),
            redirect_uri: redirect_uri.into(),
            scopes: scopes.into(),
            state: None,
            code_verifier: None,
        }
    }

    pub(crate) fn set_scopes(&mut self, scopes: Option<HashSet<Scope>>) {
        self.scopes = scopes;
    }

    pub(crate) fn user_authorization_url(&mut self) -> String {
        let code_verifier = crypto::generate_code_verifier(128);
        let code_challenge = crypto::generate_code_challenge(&code_verifier);
        let state = crypto::random_string(16);

        let mut params = QueryParams::default();
        params
            .push("client_id", &self.client_id)
            .push("response_type", &"code")
            .push("redirect_uri", &self.redirect_uri)
            .push("state", &state)
            .push_opt("scope", self.scopes.as_ref().map(scopes::to_string))
            .push("code_challenge_method", &"S256")
            .push("code_challenge", &code_challenge);

        let mut url =
            Url::parse("https://accounts.spotify.com/authorize").expect("This URL is always valid");

        params.add_to_url(&mut url);

        self.state = Some(state);
        self.code_verifier = Some(code_verifier);

        url.as_str().to_owned()
    }

    pub(crate) fn verify_authorization_code(&self, url: &str) -> AuthResult<String> {
        let self_state = self.state.as_ref().ok_or(AuthError::NoState)?;

        let url = Url::parse(url)?;

        let mut code = None;
        let mut state = None;

        for (key, value) in url.query_pairs() {
            match key.as_ref() {
                "code" => code = Some(value),
                "state" => state = Some(value),
                _ => {}
            }
        }

        let code = code.ok_or(AuthError::CodeNotFound)?;
        let state = state.ok_or(AuthError::InvalidState {
            expected: self_state.to_owned(),
            got: "None".to_owned(),
        })?;

        if self_state.eq(&state) {
            Ok(code.to_string())
        } else {
            Err(AuthError::InvalidState {
                expected: self_state.to_owned(),
                got: state.to_string(),
            })
        }
    }

    pub(crate) fn request_token(
        &self,
        code: &str,
        client: &Client,
    ) -> Result<Token, ApiError<RestError>> {
        let code_verifier = self
            .code_verifier
            .as_ref()
            .ok_or(AuthError::NoCodeVerifier)?;
        let params = self.token_request_params(code, code_verifier);
        super::request_token(client, None, params)
    }

    pub(crate) async fn request_token_async(
        &self,
        code: &str,
        client: &reqwest::Client,
    ) -> Result<Token, ApiError<RestError>> {
        let code_verifier = self
            .code_verifier
            .as_ref()
            .ok_or(AuthError::NoCodeVerifier)?;
        let params = self.token_request_params(code, code_verifier);
        super::request_token_async(client, None, params).await
    }

    pub(crate) fn request_token_from_redirect_url(
        &self,
        url: &str,
        client: &Client,
    ) -> Result<Token, ApiError<RestError>> {
        let code = self.verify_authorization_code(url)?;
        let code_verifier = self
            .code_verifier
            .as_ref()
            .ok_or(AuthError::NoCodeVerifier)?;
        let params = self.token_request_params(&code, code_verifier);
        super::request_token(client, None, params)
    }

    pub(crate) async fn request_token_from_redirect_url_async(
        &self,
        url: &str,
        client: &reqwest::Client,
    ) -> Result<Token, ApiError<RestError>> {
        let code = self.verify_authorization_code(url)?;
        let code_verifier = self
            .code_verifier
            .as_ref()
            .ok_or(AuthError::NoCodeVerifier)?;
        let params = self.token_request_params(&code, code_verifier);
        super::request_token_async(client, None, params).await
    }

    fn token_request_params<'a>(&self, code: &'a str, code_verifier: &'a str) -> FormParams<'a> {
        let mut params = FormParams::default();
        params.push("grant_type", &"authorization_code");
        params.push("code", &code);
        params.push("redirect_uri", &self.redirect_uri);
        params.push("client_id", &self.client_id);
        params.push("code_verifier", &code_verifier);
        params
    }

    fn refresh_token_request_params<'a>(&self, refresh_token: &'a str) -> FormParams<'a> {
        let mut params = FormParams::default();
        params.push("grant_type", &"refresh_token");
        params.push("refresh_token", &refresh_token);
        params.push("client_id", &self.client_id);
        params
    }
}

impl AuthFlow for AuthCodePKCE {}

impl Refresh for AuthCodePKCE {
    fn refresh_token(
        &self,
        client: &Client,
        refresh_token: &str,
    ) -> Result<Token, ApiError<RestError>> {
        let params = self.refresh_token_request_params(refresh_token);
        let (req, data) = super::http_request_and_data(None, params)?;
        let rsp = super::http_response(client, req, data).map_err(ApiError::client)?;
        super::parse_response(&rsp)
    }
}

#[async_trait]
impl AsyncRefresh for AuthCodePKCE {
    async fn refresh_token_async(
        &self,
        client: &reqwest::Client,
        refresh_token: &str,
    ) -> Result<Token, ApiError<RestError>> {
        let params = self.refresh_token_request_params(refresh_token);
        let (req, data) = super::http_request_and_data(None, params)?;
        let rsp = super::http_response_async(client, req, data)
            .await
            .map_err(ApiError::client)?;
        super::parse_response(&rsp)
    }
}

mod crypto {
    use base64::{Engine as _, engine::general_purpose};
    use rand::Rng as _;
    use sha2::{Digest, Sha256};

    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._~";

    pub fn generate_code_verifier(length: usize) -> String {
        let length = length.clamp(43, 128);
        random_string(length)
    }

    pub fn generate_code_challenge(code_verifier: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(code_verifier.as_bytes());
        let result = hasher.finalize();
        general_purpose::URL_SAFE_NO_PAD.encode(result)
    }

    pub fn random_string(length: usize) -> String {
        let mut rng = rand::rng();
        let s: String = (0..length)
            .map(|_| {
                let idx = rng.random_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();

        s
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn random_string() {
        let length = 16;
        let random_string = super::crypto::random_string(length);
        assert_eq!(random_string.len(), length);
    }
}
