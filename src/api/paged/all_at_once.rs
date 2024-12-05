use super::{Pageable, Pagination};
use crate::{
    api::{query, ApiError, AsyncClient, AsyncQuery, Client, Endpoint, Query},
    model::Page,
};
use async_trait::async_trait;
use http::{header, Request};
use parking_lot::Mutex;
use serde::de::DeserializeOwned;
use std::sync::Arc;
use url::Url;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Paged<E> {
    pub(crate) endpoint: E,
    pub(crate) pagination: Pagination,
}

/// Collect data from a paged endpoint.
pub fn paged<E>(endpoint: E, pagination: Pagination) -> Paged<E> {
    Paged {
        endpoint,
        pagination,
    }
}

/// Collect all data from a paged endpoint.
pub fn paged_all<E>(endpoint: E) -> Paged<E> {
    paged(endpoint, Pagination::All)
}

/// Collect a limited amount of data from a paged endpoint.
pub fn paged_with_limit<E>(endpoint: E, limit: usize) -> Paged<E> {
    paged(endpoint, Pagination::Limit(limit))
}

impl<E, T, C> Query<Vec<T>, C> for Paged<E>
where
    E: Endpoint + Pageable,
    T: DeserializeOwned + 'static,
    C: Client,
{
    fn query(&self, client: &C) -> Result<Vec<T>, ApiError<C::Error>> {
        self.iter(client).collect()
    }
}

#[async_trait]
impl<E, T, C> AsyncQuery<Vec<T>, C> for Paged<E>
where
    E: Endpoint + Sync,
    E: Pageable,
    T: DeserializeOwned + Send + 'static,
    C: AsyncClient + Sync,
{
    async fn query_async(&self, client: &C) -> Result<Vec<T>, ApiError<C::Error>> {
        let url = {
            let mut url = self
                .endpoint
                .url_base()
                .endpoint_for(client, &self.endpoint.endpoint())?;
            self.endpoint.parameters().add_to_url(&mut url);
            url
        };

        let results = Arc::new(Mutex::new(Vec::new()));
        let body = self.endpoint.body()?;
        let mut next_url = None;

        loop {
            let page_url = next_url.take().unwrap_or_else(|| {
                let mut page_url = url.clone();
                {
                    let mut pairs = page_url.query_pairs_mut();
                    pairs.append_pair("offset", "0");
                    pairs.append_pair("limit", &self.pagination.limit().to_string());
                }
                page_url
            });

            let (mime, data) = body.as_ref().map_or((None, Vec::new()), |(mime, data)| {
                (Some(mime), data.clone())
            });

            let req = Request::builder()
                .method(self.endpoint.method())
                .uri(query::url_to_http_uri(&page_url));

            let req = if let Some(mime) = mime {
                req.header(header::CONTENT_TYPE, *mime)
            } else {
                req
            };

            let rsp = client.rest_async(req, data).await?;
            let status = rsp.status();

            let v = serde_json::from_slice(rsp.body())
                .map_err(|_e| ApiError::server_error(status, rsp.body()))?;

            if !status.is_success() {
                return Err(ApiError::from_spotify_with_status(status, v));
            } else if status == http::StatusCode::MOVED_PERMANENTLY {
                return Err(ApiError::moved_permanently(
                    rsp.headers().get(header::LOCATION),
                ));
            }

            let page: Page<T> =
                serde_json::from_value(v).map_err(ApiError::data_type::<Page<T>>)?;

            let page_len = page.items.len();
            next_url = page.next.as_ref().map(|url| Url::parse(url)).transpose()?;

            let mut locked_results = results.lock();
            locked_results.extend(page.items);

            if self.pagination.is_last_page(page_len, locked_results.len()) || next_url.is_none() {
                break;
            }
        }

        let mut locked_results = results.lock();

        Ok(std::mem::take(&mut locked_results))
    }
}
