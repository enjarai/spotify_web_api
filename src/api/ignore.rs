use crate::api::{query, ApiError, AsyncClient, AsyncQuery, Client, Endpoint, Query};
use async_trait::async_trait;
use http::{
    header::{self, LOCATION},
    Request,
};

/// A query modifier that ignores the data returned from an endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ignore<E> {
    endpoint: E,
}

/// Ignore the resulting data from an endpoint.
pub fn ignore<E>(endpoint: E) -> Ignore<E> {
    Ignore { endpoint }
}

impl<E, C> Query<(), C> for Ignore<E>
where
    E: Endpoint,
    C: Client,
{
    fn query(&self, client: &C) -> Result<(), ApiError<C::Error>> {
        let mut url = self
            .endpoint
            .url_base()
            .endpoint_for(client, &self.endpoint.endpoint())?;

        self.endpoint.parameters().add_to_url(&mut url);

        let req = Request::builder()
            .method(self.endpoint.method())
            .uri(query::url_to_http_uri(&url));

        let (req, data) = if let Some((mime, data)) = self.endpoint.body()? {
            let req = req.header(header::CONTENT_TYPE, mime);
            (req, data)
        } else {
            (req, Vec::new())
        };

        let rsp = client.rest(req, data)?;
        let status = rsp.status();

        if !status.is_success() {
            let v = serde_json::from_slice(rsp.body())
                .map_err(|_e| ApiError::server_error(status, rsp.body()))?;
            return Err(ApiError::from_spotify_with_status(status, v));
        } else if status == http::StatusCode::MOVED_PERMANENTLY {
            return Err(ApiError::moved_permanently(rsp.headers().get(LOCATION)));
        }

        Ok(())
    }
}

#[async_trait]
impl<E, C> AsyncQuery<(), C> for Ignore<E>
where
    E: Endpoint + Sync,
    C: AsyncClient + Sync,
{
    async fn query_async(&self, client: &C) -> Result<(), ApiError<C::Error>> {
        let mut url = self
            .endpoint
            .url_base()
            .endpoint_for(client, &self.endpoint.endpoint())?;

        self.endpoint.parameters().add_to_url(&mut url);

        let req = Request::builder()
            .method(self.endpoint.method())
            .uri(query::url_to_http_uri(&url));

        let (req, data) = if let Some((mime, data)) = self.endpoint.body()? {
            let req = req.header(header::CONTENT_TYPE, mime);
            (req, data)
        } else {
            (req, Vec::new())
        };

        let rsp = client.rest_async(req, data).await?;
        let status = rsp.status();

        if !status.is_success() {
            let v = serde_json::from_slice(rsp.body())
                .map_err(|_e| ApiError::server_error(status, rsp.body()))?;
            return Err(ApiError::from_spotify_with_status(status, v));
        } else if status == http::StatusCode::MOVED_PERMANENTLY {
            return Err(ApiError::moved_permanently(rsp.headers().get(LOCATION)));
        }

        Ok(())
    }
}
