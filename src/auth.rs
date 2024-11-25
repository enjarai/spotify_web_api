// FLOW		                     Access User Resources		Requires Secret Key (Server-Side)		Access Token Refresh
// --------------------------------------------------------------------------------------------------------------
// Authorization code with PKCE	 Yes	                    No	                                    Yes
// Client credentials	         No	                        Yes	                                    No

mod client_credentials;
mod pkce;
pub mod scopes;

use crate::{
    api::{query, ApiError, FormParams},
    model::Token,
    RestError,
};
use bytes::Bytes;
pub use client_credentials::ClientCredentials;
use http::{request::Builder, HeaderMap, HeaderValue, Request, Response as HttpResponse};
pub use pkce::AuthCodePKCE;
use reqwest::blocking::Client;
use thiserror::Error;
use url::Url;

pub type AuthResult<T> = Result<T, AuthError>;

/// Represents errors that can occur during the authorization process.
///
/// This enum defines various error conditions that may arise while handling OAuth
/// authorization flows, such as invalid headers, URL parsing issues, or mismatched
/// state parameters.
///
/// # Variants
/// - `HeaderValue`: An error related to an invalid HTTP header value.
/// - `UrlParse`: An error that occurs when parsing a URL fails.
/// - `CodeNotFound`: Indicates that the authorization code was not found in the URL.
/// - `InvalidState`: Indicates a mismatch between the expected and received state parameters.
///
/// This enum is marked as `#[non_exhaustive]`, meaning new variants may be added in future versions.
/// When matching against it, include a wildcard arm (`_`) to account for any future variants.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AuthError {
    /// An error related to an invalid HTTP header value.
    ///
    /// # Details
    /// This variant wraps an `http::header::InvalidHeaderValue` error, which occurs
    /// when constructing an HTTP header fails due to invalid characters or formatting.
    #[error("header value error: {source}")]
    HeaderValue {
        /// The source of the error.
        #[from]
        source: http::header::InvalidHeaderValue,
    },

    /// The URL failed to parse.
    ///
    /// # Details
    /// This variant wraps a `url::ParseError`, which occurs when a URL string
    /// cannot be correctly parsed.
    #[error("failed to parse url: {source}")]
    UrlParse {
        /// The source of the error.
        #[from]
        source: url::ParseError,
    },

    /// Indicates that the authorization code was not found in the URL.
    ///
    /// This error is returned when the query string of a URL does not contain
    /// the expected `code` parameter during an OAuth callback.
    #[error("authorization code not found")]
    CodeNotFound,

    /// Indicates a mismatch between the expected and received state parameters.
    ///
    /// The state parameter is used in OAuth to prevent CSRF attacks. This error
    /// occurs when the state value returned by the authorization server does not
    /// match the expected value.
    ///
    /// # Fields
    /// - `expected`: The expected state parameter.
    /// - `got`: The state parameter that was received.
    #[error("invalid state parameter: expected {expected} got {got}")]
    InvalidState { expected: String, got: String },

    /// Indicates that the state parameter is missing.
    ///
    /// This error occurs when the state value is expected but not present.
    /// Ensure that the `user_authorization_url()` method is called to generate
    /// a proper authorization URL that includes the state parameter.
    #[error("AuthCodePKCE's state is None. Make sure to generate a user authorization URL by calling user_authorization_url()")]
    NoState,

    /// Indicates that the code verifier is missing.
    ///
    /// This error occurs when the code verifier is expected but not present.
    /// Ensure that the `user_authorization_url()` method is called to generate
    /// the code verifier required for the PKCE flow.
    #[error("AuthCodePKCE's code_verifier is None. Make sure to generate a code verifier by calling user_authorization_url()")]
    NoCodeVerifier,

    /// Represents an error when the access token is empty, indicating that
    /// authentication cannot proceed without a valid token.
    #[error("access token is empty")]
    EmptyAccessToken,
}

pub(crate) mod private {
    pub trait AuthFlow {}
}

pub(super) fn request_token(
    client: &Client,
    auth_header_value: Option<String>,
    params: FormParams<'_>,
) -> Result<Token, ApiError<RestError>> {
    let (req, data) = http_request_and_data(auth_header_value, params)?;
    let rsp = http_response(client, req, data).map_err(ApiError::client)?;
    parse_response(&rsp)
}

pub(super) async fn request_token_async(
    client: &reqwest::Client,
    auth_header_value: Option<String>,
    params: FormParams<'_>,
) -> Result<Token, ApiError<RestError>> {
    let (req, data) = http_request_and_data(auth_header_value, params)?;
    let rsp = http_response_async(client, req, data)
        .await
        .map_err(ApiError::client)?;
    parse_response(&rsp)
}

fn set_auth_header<'a>(
    headers: &'a mut HeaderMap<HeaderValue>,
    value: &str,
) -> AuthResult<&'a mut HeaderMap<HeaderValue>> {
    let mut header_value = HeaderValue::from_str(value)?;
    header_value.set_sensitive(true);
    headers.insert(http::header::AUTHORIZATION, header_value);
    Ok(headers)
}

#[inline(always)]
fn http_request_and_data(
    auth_header_value: Option<String>,
    params: FormParams<'_>,
) -> Result<(Builder, Vec<u8>), ApiError<RestError>> {
    let url = Url::parse("https://accounts.spotify.com/api/token")?;

    let mut req = Request::builder()
        .method(http::Method::POST)
        .uri(query::url_to_http_uri(&url));

    if let Some(value) = auth_header_value {
        set_auth_header(
            req.headers_mut()
                .expect("failed to get headers on the request builder"),
            &value,
        )?;
    }

    let value = if let Some((mime, data)) = params.into_body()? {
        let req = req.header(http::header::CONTENT_TYPE, mime);
        (req, data)
    } else {
        (req, Vec::new())
    };

    Ok(value)
}

#[inline(always)]
fn http_response(
    client: &Client,
    req: Builder,
    data: Vec<u8>,
) -> Result<http::Response<Bytes>, RestError> {
    let http_request = req.body(data)?;
    let request = http_request.try_into()?;
    let rsp = client.execute(request)?;
    let mut http_rsp = HttpResponse::builder()
        .status(rsp.status())
        .version(rsp.version());

    let headers = http_rsp
        .headers_mut()
        .expect("failed to get headers on the request builder");

    for (key, value) in rsp.headers() {
        headers.insert(key, value.clone());
    }

    Ok(http_rsp.body(rsp.bytes()?)?)
}

#[inline(always)]
async fn http_response_async(
    client: &reqwest::Client,
    req: Builder,
    data: Vec<u8>,
) -> Result<http::Response<Bytes>, RestError> {
    let http_request = req.body(data)?;
    let request = http_request.try_into()?;
    let rsp = client.execute(request).await?;
    let mut http_rsp = HttpResponse::builder()
        .status(rsp.status())
        .version(rsp.version());

    let headers = http_rsp
        .headers_mut()
        .expect("failed to get headers on the request builder");

    for (key, value) in rsp.headers() {
        headers.insert(key, value.clone());
    }

    Ok(http_rsp.body(rsp.bytes().await?)?)
}

#[inline(always)]
fn parse_response(rsp: &http::Response<Bytes>) -> Result<Token, ApiError<RestError>> {
    let status = rsp.status();

    let Ok(v) = serde_json::from_slice(rsp.body()) else {
        return Err(ApiError::server_error(status, rsp.body()));
    };

    if !status.is_success() {
        return Err(ApiError::from_spotify_with_status(status, v));
    } else if status == http::StatusCode::MOVED_PERMANENTLY {
        return Err(ApiError::moved_permanently(
            rsp.headers().get(http::header::LOCATION),
        ));
    }

    serde_json::from_value::<_>(v).map_err(ApiError::data_type::<Token>)
}
