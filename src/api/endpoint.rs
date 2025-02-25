use super::{
    client::{AsyncClient, Client, RestClient},
    error::{ApiError, BodyError},
    params::QueryParams,
    query::{self, AsyncQuery, Query},
};
use async_trait::async_trait;
use http::{
    Method, Request,
    header::{self, LOCATION},
};
use serde::de::DeserializeOwned;
use std::borrow::Cow;
use url::Url;

/// URL bases for endpoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum UrlBase {
    /// An endpoint uses the API v1 URL prefix.
    ApiV1,
}

impl UrlBase {
    /// Get the endpoint for a given URL base.
    pub fn endpoint_for<C>(&self, client: &C, endpoint: &str) -> Result<Url, ApiError<C::Error>>
    where
        C: RestClient,
    {
        match self {
            Self::ApiV1 => client.rest_endpoint(endpoint),
        }
    }
}

pub trait Endpoint {
    /// The HTTP method to use for the endpoint.
    fn method(&self) -> Method;

    /// The path to the endpoint.
    fn endpoint(&self) -> Cow<'static, str>;

    /// The URL base of the API endpoint.
    fn url_base(&self) -> UrlBase {
        UrlBase::ApiV1
    }

    /// Query parameters for the endpoint.
    fn parameters(&self) -> QueryParams<'_> {
        QueryParams::default()
    }

    /// The body for the endpoint.
    ///
    /// Returns the `Content-Encoding` header for the data as well as the data itself.
    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        Ok(None)
    }
}

impl<E> Endpoint for &E
where
    E: Endpoint,
{
    fn method(&self) -> Method {
        (*self).method()
    }

    fn endpoint(&self) -> Cow<'static, str> {
        (*self).endpoint()
    }

    fn url_base(&self) -> UrlBase {
        (*self).url_base()
    }

    fn parameters(&self) -> QueryParams<'_> {
        (*self).parameters()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        (*self).body()
    }
}

impl<E, T, C> Query<T, C> for E
where
    E: Endpoint,
    T: DeserializeOwned,
    C: Client,
{
    fn query(&self, client: &C) -> Result<T, ApiError<C::Error>> {
        let mut url = self.url_base().endpoint_for(client, &self.endpoint())?;

        self.parameters().add_to_url(&mut url);

        let (mime, data) = self.body()?.map_or((None, Vec::new()), |(mime, data)| {
            (Some(mime), data.clone())
        });

        let mut req = Request::builder()
            .method(self.method())
            .uri(query::url_to_http_uri(&url));

        if let Some(mime) = mime {
            req = req.header(header::CONTENT_TYPE, mime);
        }

        if matches!(self.method(), Method::POST | Method::PUT) {
            req = req.header(header::CONTENT_LENGTH, data.len().to_string());
        }

        let rsp = client.rest(req, data)?;
        let status = rsp.status();

        let v = serde_json::from_slice(rsp.body())
            .map_err(|_e| ApiError::server_error(status, rsp.body()))?;

        if !status.is_success() {
            return Err(ApiError::from_spotify_with_status(status, v));
        } else if status == http::StatusCode::MOVED_PERMANENTLY {
            return Err(ApiError::moved_permanently(rsp.headers().get(LOCATION)));
        }

        serde_json::from_value::<T>(v).map_err(ApiError::data_type::<T>)
    }
}

#[async_trait]
impl<E, T, C> AsyncQuery<T, C> for E
where
    E: Endpoint + Sync,
    T: DeserializeOwned + 'static,
    C: AsyncClient + Sync,
{
    async fn query_async(&self, client: &C) -> Result<T, ApiError<C::Error>> {
        let mut url = self.url_base().endpoint_for(client, &self.endpoint())?;

        self.parameters().add_to_url(&mut url);

        let (mime, data) = self.body()?.map_or((None, Vec::new()), |(mime, data)| {
            (Some(mime), data.clone())
        });

        let mut req = Request::builder()
            .method(self.method())
            .uri(query::url_to_http_uri(&url));

        if let Some(mime) = mime {
            req = req.header(header::CONTENT_TYPE, mime);
        }

        if matches!(self.method(), Method::POST | Method::PUT) {
            req = req.header(header::CONTENT_LENGTH, data.len().to_string());
        }

        let rsp = client.rest_async(req, data).await?;
        let status = rsp.status();

        let v = serde_json::from_slice(rsp.body())
            .map_err(|_e| ApiError::server_error(status, rsp.body()))?;

        if !status.is_success() {
            return Err(ApiError::from_spotify_with_status(status, v));
        } else if status == http::StatusCode::MOVED_PERMANENTLY {
            return Err(ApiError::moved_permanently(rsp.headers().get(LOCATION)));
        }

        serde_json::from_value::<T>(v).map_err(ApiError::data_type::<T>)
    }
}
