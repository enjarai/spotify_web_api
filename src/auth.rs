use thiserror::Error;

pub type AuthResult<T> = Result<T, AuthError>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AuthError {
    #[error("header value error: {}", source)]
    HeaderValue {
        #[from]
        source: http::header::InvalidHeaderValue,
    },
}
