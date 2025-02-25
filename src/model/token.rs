use chrono::{DateTime, TimeDelta, Utc, serde::ts_seconds_option};
use serde::{Deserialize, Serialize};

/// Represents an OAuth 2.0 access token for authenticating API requests.
///
/// This struct contains the details of the access token received from an
/// OAuth 2.0 authorization process, such as the token itself, its expiration
/// time, a refresh token, and the scopes granted to the token. The `Token`
/// struct is used to interact with APIs that require OAuth 2.0 authentication
/// (such as the Spotify Web API).
///
/// # Fields
/// - `access_token`: The access token itself, which is used to authenticate
///   requests to the API.
/// - `token_type`: The type of token, usually "Bearer".
/// - `expires_in`: The time period (in seconds) for which the access token is valid.
/// - `expires_at`: An optional field that stores the expiration time of the token,
///   used to check if the token has expired. If not provided, the expiration time
///   is inferred from `expires_in`.
/// - `refresh_token`: An optional field that contains a refresh token used to
///   obtain new access tokens without user reauthorization.
/// - `scope`: An optional field that contains a list of scopes granted to the
///   `access_token`.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Token {
    /// An access token that can be provided in subsequent calls, for example to Spotify Web API services.
    pub access_token: String,

    /// How the access token may be used: always "Bearer".
    pub token_type: String,

    /// The time period (in seconds) for which the access token is valid.
    /// Access tokens are intentionally configured to have a limited lifespan (1 hour),
    /// at the end of which, new tokens can be obtained by providing the original `refresh_token` acquired
    /// during the authorization token request response.
    pub expires_in: u16,

    /// Not part of the OAuth 2.0 Access Token, but used to store it's expiration time.
    #[serde(default)]
    #[serde(with = "ts_seconds_option")]
    pub expires_at: Option<DateTime<Utc>>,

    /// Security credential that allows client applications to obtain new access tokens without requiring users to reauthorize the application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,

    /// A space-separated list of scopes which have been granted for this `access_token`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

impl Token {
    /// Checks if the access token has expired.
    ///
    /// This method compares the current time with the `expires_at` field (if present).
    /// It allows for a small margin (10 seconds) before considering the token as expired.
    ///
    /// If `expires_at` is not set, it assumes the token is expired.
    ///
    /// # Returns
    /// - `true`: If the token is expired or the expiration time is not set.
    /// - `false`: If the token is still valid.
    pub fn is_expired(&self) -> bool {
        self.expires_at
            .is_none_or(|expires_at| Utc::now() + TimeDelta::seconds(10) >= expires_at)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token() {
        let json = r#"
        {
			"access_token": "string",
			"token_type": "Bearer",
			"expires_in": 3600,
			"refresh_token": "string",
			"scope": "user-read-email user-read-private"
        }
        "#;

        crate::test::assert_deserialized!(Token, json);
    }
}
