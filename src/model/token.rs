use chrono::{serde::ts_seconds_option, DateTime, TimeDelta, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Token {
    /// An access token that can be provided in subsequent calls, for example to Spotify Web API services.
    pub access_token: String,

    /// How the access token may be used: always "Bearer".
    pub token_type: String,

    /// The time period (in seconds) for which the access token is valid.
    pub expires_in: u16,

    /// Not part of the OAuth 2.0 Access Token, but used to store the expiration time of the token.
    #[serde(default)]
    #[serde(with = "ts_seconds_option")]
    pub expires_at: Option<DateTime<Utc>>,

    /// Security credential that allows client applications to obtain new access tokens without requiring users to reauthorize the application.
    pub refresh_token: String,

    // TODO(ricky): This should be a HashSet<Scope> which requires a custom deserializer.
    /// A space-separated list of scopes which have been granted for this `access_token`.
    pub scope: String,
}

impl Token {
    pub fn is_expired(&self) -> bool {
        self.expires_at.map_or(true, |expires_at| {
            Utc::now() + TimeDelta::seconds(10) >= expires_at
        })
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
