use super::endpoint::UrlBase;
use crate::auth::AuthError;
use std::error::Error;
use thiserror::Error;

/// Errors which may occur when creating form data.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum BodyError {
    /// Body data could not be serialized from form parameters.
    #[error("failed to URL encode form parameters: {}", source)]
    UrlEncoded {
        /// The source of the error.
        #[from]
        source: serde_urlencoded::ser::Error,
    },

    /// Body data could not be serialized to JSON from form parameters.
    #[error("failed to JSON encode form parameters: {}", source)]
    JsonEncoded {
        /// The source of the error.
        #[from]
        source: serde_json::Error,
    },
}

/// Errors which may occur when using API endpoints.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ApiError<E>
where
    E: Error + Send + Sync + 'static,
{
    /// The client encountered an error.
    #[error("client error: {}", source)]
    Client {
        /// The client error.
        source: E,
    },

    /// Authentication failed.
    #[error("failed to authenticate: {}", source)]
    Auth {
        /// The source of the error.
        #[from]
        source: AuthError,
    },

    /// The URL failed to parse.
    #[error("failed to parse url: {}", source)]
    UrlParse {
        /// The source of the error.
        #[from]
        source: url::ParseError,
    },

    /// Body data could not be created.
    #[error("failed to create form data: {}", source)]
    Body {
        /// The source of the error.
        #[from]
        source: BodyError,
    },

    /// JSON deserialization from Spotify failed.
    #[error("could not parse JSON response: {}", source)]
    Json {
        /// The source of the error.
        #[from]
        source: serde_json::Error,
    },

    /// The resource has been moved permanently.
    #[error("moved permanently to: {}", location.as_ref().map_or("<UNKNOWN>", AsRef::as_ref))]
    MovedPermanently {
        /// The new location for the resource.
        location: Option<String>,
    },

    /// Spotify returned an error without JSON information.
    #[error("spotify internal server error {}", status)]
    SpotifyService {
        /// The status code for the return.
        status: http::StatusCode,

        /// The error data from Spotify.
        data: Vec<u8>,
    },

    /// Failed to parse an expected data type from JSON.
    #[error("could not parse {} data from JSON: {}", typename, source)]
    DataType {
        /// The source of the error.
        source: serde_json::Error,

        /// The name of the type that could not be deserialized.
        typename: &'static str,
    },

    /// The client does not understand how to use an endpoint for the given URL base.
    #[error("unsupported URL base: {:?}", url_base)]
    UnsupportedUrlBase {
        /// The URL base that is not supported.
        url_base: UrlBase,
    },

    /// Spotify returned an error message with an HTTP error.
    #[error("spotify server error ({}): {}", status, msg)]
    SpotifyWithStatus {
        /// The HTTP status code.
        status: http::StatusCode,

        /// The error message from Spotify.
        msg: String,
    },

    /// Spotify returned an error object with an HTTP error.
    #[error("spotify server error ({}): {:?}", status, obj)]
    SpotifyObjectWithStatus {
        /// The HTTP status code.
        status: http::StatusCode,

        /// The error object from Spotify.
        obj: serde_json::Value,
    },

    /// Spotify returned an HTTP error with JSON we did not recognize.
    #[error("spotify server error ({}): {:?}", status, obj)]
    SpotifyUnrecognizedWithStatus {
        /// The HTTP status code.
        status: http::StatusCode,

        /// The full object from Spotify.
        obj: serde_json::Value,
    },
}

impl<E> ApiError<E>
where
    E: Error + Send + Sync + 'static,
{
    /// Create an API error in a client error.
    pub fn client(source: E) -> Self {
        Self::Client { source }
    }

    /// Wrap a client error in another wrapper.
    pub fn map_client<F, W>(self, f: F) -> ApiError<W>
    where
        F: FnOnce(E) -> W,
        W: Error + Send + Sync + 'static,
    {
        match self {
            Self::Client { source } => ApiError::client(f(source)),
            Self::UrlParse { source } => ApiError::UrlParse { source },
            Self::Auth { source } => ApiError::Auth { source },
            Self::Body { source } => ApiError::Body { source },
            Self::Json { source } => ApiError::Json { source },
            Self::MovedPermanently { location } => ApiError::MovedPermanently { location },
            Self::SpotifyWithStatus { status, msg } => ApiError::SpotifyWithStatus { status, msg },
            Self::SpotifyService { status, data } => ApiError::SpotifyService { status, data },
            Self::SpotifyObjectWithStatus { status, obj } => {
                ApiError::SpotifyObjectWithStatus { status, obj }
            }
            Self::SpotifyUnrecognizedWithStatus { status, obj } => {
                ApiError::SpotifyUnrecognizedWithStatus { status, obj }
            }
            Self::DataType { source, typename } => ApiError::DataType { source, typename },
            Self::UnsupportedUrlBase { url_base } => ApiError::UnsupportedUrlBase { url_base },
        }
    }

    pub(crate) fn moved_permanently(raw_location: Option<&http::HeaderValue>) -> Self {
        let location = raw_location.map(|v| String::from_utf8_lossy(v.as_bytes()).into());
        Self::MovedPermanently { location }
    }

    pub(crate) fn server_error(status: http::StatusCode, body: &bytes::Bytes) -> Self {
        Self::SpotifyService {
            status,
            data: body.into_iter().copied().collect(),
        }
    }

    pub(crate) fn from_spotify_with_status(
        status: http::StatusCode,
        value: serde_json::Value,
    ) -> Self {
        let error_value = value
            .pointer("/message")
            .or_else(|| value.pointer("/error"));

        if let Some(error_value) = error_value {
            if let Some(msg) = error_value.as_str() {
                Self::SpotifyWithStatus {
                    status,
                    msg: msg.into(),
                }
            } else {
                Self::SpotifyObjectWithStatus {
                    status,
                    obj: error_value.clone(),
                }
            }
        } else {
            Self::SpotifyUnrecognizedWithStatus { status, obj: value }
        }
    }

    pub(crate) fn data_type<T>(source: serde_json::Error) -> Self {
        Self::DataType {
            source,
            typename: std::any::type_name::<T>(),
        }
    }
}
