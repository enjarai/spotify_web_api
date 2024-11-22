use super::{
    client::{AsyncClient, Client},
    error::ApiError,
};
use async_trait::async_trait;

pub fn url_to_http_uri(url: &url::Url) -> http::Uri {
    url.as_str()
        .parse::<http::Uri>()
        .expect("failed to parse a url::Url as an http::Uri")
}

/// A trait which represents a query which may be made to a Spotify client.
pub trait Query<T, C>
where
    C: Client,
{
    /// Perform the query against the client.
    fn query(&self, client: &C) -> Result<T, ApiError<C::Error>>;
}

/// A trait which represents an asynchronous query which may be made to a Spotify client.
#[async_trait]
pub trait AsyncQuery<T, C>
where
    C: AsyncClient,
{
    /// Perform the query asynchronously against the client.
    async fn query_async(&self, client: &C) -> Result<T, ApiError<C::Error>>;
}
