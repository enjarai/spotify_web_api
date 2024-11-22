use crate::{
    api::{self, RestClient},
    AuthError,
};
use reqwest::blocking::Client;
use std::time::Duration;
use thiserror::Error;
use url::Url;

const BASE_API_URL: &str = "https://api.spotify.com/v1";

pub type SpotifyResult<T> = Result<T, SpotifyError>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum RestError {
    #[error("error setting auth header: {}", source)]
    AuthError {
        #[from]
        source: AuthError,
    },

    #[error("communication with spotify: {}", source)]
    Communication {
        #[from]
        source: reqwest::Error,
    },

    #[error("`http` error: {}", source)]
    Http {
        #[from]
        source: http::Error,
    },
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum SpotifyError {
    #[error("failed to parse url: {}", source)]
    UrlParse {
        #[from]
        source: url::ParseError,
    },

    #[error("error setting auth header: {}", source)]
    AuthError {
        #[from]
        source: AuthError,
    },

    #[error("communication with spotify: {}", source)]
    Communication {
        #[from]
        source: reqwest::Error,
    },

    #[error("spotify HTTP error: {}", status)]
    Http { status: reqwest::StatusCode },

    #[error("no response from spotify")]
    NoResponse,

    #[error("could not parse {} data from JSON: {}", typename, source)]
    DataType {
        #[source]
        source: serde_json::Error,
        typename: &'static str,
    },

    #[error("api error: {}", source)]
    Api {
        #[from]
        source: api::ApiError<RestError>,
    },
}

impl SpotifyError {
    fn http(status: reqwest::StatusCode) -> Self {
        Self::Http { status }
    }

    fn no_response() -> Self {
        Self::NoResponse
    }

    fn data_type<T>(source: serde_json::Error) -> Self {
        Self::DataType {
            source,
            typename: std::any::type_name::<T>(),
        }
    }
}

pub struct Spotify {
    /// The client to use for API calls.
    client: Client,

    /// The base URL to use for API calls.
    api_url: Url,
}

impl Spotify {
    pub fn new() -> SpotifyResult<Self> {
        let api_url = Url::parse(BASE_API_URL)?;
        let client = Client::builder().timeout(Duration::from_secs(10)).build()?;
        let api = Self { client, api_url };
        Ok(api)
    }
}

impl RestClient for Spotify {
    type Error = RestError;

    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, api::ApiError<Self::Error>> {
        log::debug!("REST api call {endpoint}");
        Ok(self.api_url.join(endpoint)?)
    }
}
