use self::query::{AsyncQuery, Query};
use super::{Pageable, Paged, Pagination};
use crate::{
    api::{ApiError, AsyncClient, Client, Endpoint, RestClient, query},
    model::Page,
};
use async_trait::async_trait;
use bytes::Bytes;
use futures_util::Stream;
use http::{Method, Request, Response, header, request::Builder as RequestBuilder};
use parking_lot::RwLock;
use serde::de::DeserializeOwned;
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq)]
enum PageCursor {
    First,
    Next(Url),
    Done,
}

impl PageCursor {
    fn next_url(&self) -> Option<&Url> {
        match self {
            Self::Next(url) => Some(url),
            _ => None,
        }
    }

    fn is_done(&self) -> bool {
        matches!(self, Self::Done)
    }
}

struct PageState {
    offset: usize,
    total: usize,
    next_page: PageCursor,
}

struct LazilyPagedState<E> {
    paged: Paged<E>,
    page_state: RwLock<PageState>,
}

impl<E> LazilyPagedState<E>
where
    E: Pageable,
{
    fn new(paged: Paged<E>) -> Self {
        let offset = match paged.pagination {
            Pagination::Page { offset, .. } => offset,
            _ => 0,
        };

        let page_state = PageState {
            offset,
            total: 0,
            next_page: PageCursor::First,
        };

        Self {
            paged,
            page_state: RwLock::new(page_state),
        }
    }
}

impl<E> LazilyPagedState<E> {
    fn next_page(&self, last_page_size: usize, next_url: Option<Url>) {
        let mut page_state = self.page_state.write();
        page_state.total += last_page_size;
        page_state.next_page = if self
            .paged
            .pagination
            .is_last_page(last_page_size, page_state.total)
        {
            PageCursor::Done
        } else {
            next_url.map_or(PageCursor::Done, PageCursor::Next)
        };
    }
}

impl<E> LazilyPagedState<E>
where
    E: Endpoint,
{
    fn page_url<C>(&self, client: &C) -> Result<Option<Url>, ApiError<C::Error>>
    where
        C: RestClient,
    {
        let page_state = self.page_state.read();
        let next_page = &page_state.next_page;
        let offset = page_state.offset;

        if next_page.is_done() {
            return Ok(None);
        }

        let url = if let Some(next_url) = next_page.next_url() {
            next_url.clone()
        } else {
            let mut url = self
                .paged
                .endpoint
                .url_base()
                .endpoint_for(client, &self.paged.endpoint.endpoint())?;

            self.paged.endpoint.parameters().add_to_url(&mut url);

            url.query_pairs_mut()
                .append_pair("offset", &offset.to_string())
                .append_pair("limit", &self.paged.pagination.limit().to_string());

            url
        };

        Ok(Some(url))
    }

    fn build_request<C>(&self, url: &Url) -> Result<(RequestBuilder, Vec<u8>), ApiError<C::Error>>
    where
        C: RestClient,
    {
        let (mime, data) = self
            .paged
            .endpoint
            .body()?
            .map_or((None, Vec::new()), |(mime, data)| (Some(mime), data));

        let mut req = Request::builder()
            .method(self.paged.endpoint.method())
            .uri(query::url_to_http_uri(url));

        if let Some(mime) = mime {
            req = req.header(header::CONTENT_TYPE, mime);
        }

        if matches!(self.paged.endpoint.method(), Method::POST | Method::PUT) {
            req = req.header(header::CONTENT_LENGTH, data.len().to_string());
        }

        Ok((req, data))
    }

    fn process_response<C, T>(&self, rsp: &Response<Bytes>) -> Result<Page<T>, ApiError<C::Error>>
    where
        E: Pageable,
        T: DeserializeOwned,
        C: RestClient,
    {
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

        let page = serde_json::from_value::<Page<T>>(v).map_err(ApiError::data_type::<Page<T>>)?;

        let next_url = page.next.as_ref().map(|url| Url::parse(url)).transpose()?;

        self.next_page(page.items.len(), next_url);

        Ok(page)
    }
}

impl<E, T, C> Query<Vec<T>, C> for LazilyPagedState<E>
where
    E: Endpoint + Pageable,
    T: DeserializeOwned,
    C: Client,
{
    fn query(&self, client: &C) -> Result<Vec<T>, ApiError<C::Error>> {
        let Some(url) = self.page_url(client)? else {
            return Ok(Vec::new());
        };
        let (req, data) = self.build_request::<C>(&url)?;
        let rsp = client.rest(req, data)?;
        let page = self.process_response::<C, _>(&rsp)?;
        Ok(page.items)
    }
}

#[async_trait]
impl<E, T, C> AsyncQuery<Vec<T>, C> for LazilyPagedState<E>
where
    E: Endpoint + Pageable + Sync,
    T: DeserializeOwned + 'static,
    C: AsyncClient + Sync,
{
    async fn query_async(&self, client: &C) -> Result<Vec<T>, ApiError<C::Error>> {
        let Some(url) = self.page_url(client)? else {
            return Ok(Vec::new());
        };
        let (req, data) = self.build_request::<C>(&url)?;
        let rsp = client.rest_async(req, data).await?;
        let page = self.process_response::<C, _>(&rsp)?;
        Ok(page.items)
    }
}

/// An iterator which yields items from a paginated result.
///
/// The pages are fetched lazily, so endpoints not using offset pagination may observe duplicate or
/// missing items (depending on sorting) if new objects are created or removed while iterating.
pub struct LazilyPagedIter<'a, E, C, T> {
    client: &'a C,
    state: LazilyPagedState<E>,
    current_page: Vec<T>,
}

impl<'a, E, C, T> LazilyPagedIter<'a, E, C, T>
where
    E: Endpoint + Pageable,
{
    fn new(paged: Paged<E>, client: &'a C) -> Self {
        Self {
            client,
            state: LazilyPagedState::new(paged),
            current_page: Vec::new(),
        }
    }
}

impl<E, C, T> Iterator for LazilyPagedIter<'_, E, C, T>
where
    E: Endpoint + Pageable,
    T: DeserializeOwned,
    C: Client,
{
    type Item = Result<T, ApiError<C::Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_page.is_empty() {
            self.current_page = match self.state.query(self.client) {
                Ok(data) => data,
                Err(err) => return Some(Err(err)),
            };
            self.current_page.reverse();
        }
        self.current_page.pop().map(Ok)
    }
}

impl<'a, E, C, T> LazilyPagedIter<'a, E, C, T>
where
    E: Endpoint + Pageable + Sync,
    T: DeserializeOwned + 'static,
    C: AsyncClient + Sync,
{
    async fn next_async(&mut self) -> Option<Result<T, ApiError<C::Error>>> {
        if self.current_page.is_empty() {
            self.current_page = match self.state.query_async(self.client).await {
                Ok(data) => data,
                Err(err) => return Some(Err(err)),
            };
            self.current_page.reverse();
        }

        self.current_page.pop().map(Ok)
    }

    /// Converts a "normal iterator" into an async iterator
    pub fn into_async(self) -> impl Stream<Item = Result<T, ApiError<C::Error>>> + 'a
    where
        E: 'a,
    {
        futures_util::stream::unfold(self, |mut iter| async move {
            iter.next_async().await.map(|item| (item, iter))
        })
    }
}

impl<E> Paged<E>
where
    E: Endpoint + Pageable,
{
    /// Create an iterator over the results of paginated results for with a client.
    #[allow(clippy::iter_not_returning_iterator)]
    pub fn iter<'a, C, T>(&'a self, client: &'a C) -> LazilyPagedIter<'a, &'a E, C, T> {
        let borrowed = Paged::<&E> {
            endpoint: &self.endpoint,
            pagination: self.pagination,
        };
        LazilyPagedIter::new(borrowed, client)
    }

    /// Create an iterator over the results of paginated results for with a client.
    pub fn into_lazy_iter<C, T>(self, client: &C) -> LazilyPagedIter<'_, E, C, T> {
        LazilyPagedIter::new(self, client)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        api::{self, ApiError, Endpoint, Pagination},
        test::client::{ExpectedUrl, PagedTestClient, SingleTestClient},
    };
    use http::{Method, StatusCode};
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use std::borrow::Cow;
    use url::Url;

    #[derive(Debug, Default)]
    struct Dummy;

    impl Endpoint for Dummy {
        fn method(&self) -> Method {
            Method::GET
        }

        fn endpoint(&self) -> Cow<'static, str> {
            "paged_dummy".into()
        }
    }

    impl Pageable for Dummy {}

    #[derive(Debug, Clone, Deserialize, Serialize)]
    struct DummyResult {
        value: u8,
    }

    #[test]
    fn page_next_url() {
        let url = Url::parse("https://example.com").unwrap();
        let first = PageCursor::First;
        let next = PageCursor::Next(url.clone());
        let done = PageCursor::Done;

        assert_eq!(first.next_url(), None);
        assert_eq!(next.next_url(), Some(&url));
        assert_eq!(done.next_url(), None);
    }

    #[test]
    fn pagination_limit() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("paged_dummy")
            .paginated(true)
            .build();

        let client =
            PagedTestClient::new_raw(endpoint, (0..=255).map(|value| DummyResult { value }));

        let res: Vec<DummyResult> = api::paged(Dummy, Pagination::Limit(3))
            .iter(&client)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(res.len(), 3);

        for (i, value) in res.iter().enumerate() {
            assert_eq!(value.value, i as u8);
        }
    }

    #[test]
    fn pagination_limit_and_offset() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("paged_dummy")
            .paginated(true)
            .build();

        let client =
            PagedTestClient::new_raw(endpoint, (0..=255).map(|value| DummyResult { value }));

        let res: Vec<DummyResult> = api::paged_with_limit_and_offset(Dummy, 5, 15)
            .iter(&client)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(res.len(), 5);

        for (i, value) in res.iter().enumerate() {
            assert_eq!(value.value, (i + 15) as u8);
        }
    }

    #[test]
    fn pagination_all() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("paged_dummy")
            .paginated(true)
            .build();

        let client =
            PagedTestClient::new_raw(endpoint, (0..=55).map(|value| DummyResult { value }));

        let res: Vec<DummyResult> = api::paged(Dummy, Pagination::All)
            .iter(&client)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(res.len(), 56);

        for (i, value) in res.iter().enumerate() {
            assert_eq!(value.value, i as u8);
        }
    }

    #[test]
    fn non_json_response() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("paged_dummy")
            .add_query_params(&[("offset", "0"), ("limit", "50")])
            .build();

        let client = SingleTestClient::new_raw(endpoint, "not json");

        let res: Result<Vec<DummyResult>, _> =
            api::paged(Dummy, Pagination::All).iter(&client).collect();

        let err = res.unwrap_err();

        if let ApiError::SpotifyService { status, .. } = err {
            assert_eq!(status, StatusCode::OK);
        } else {
            panic!("unexpected error: {err}");
        }
    }

    #[test]
    fn error_bad_json() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("paged_dummy")
            .add_query_params(&[("offset", "0"), ("limit", "50")])
            .status(StatusCode::NOT_FOUND)
            .build();

        let client = SingleTestClient::new_raw(endpoint, "");

        let res: Result<Vec<DummyResult>, _> =
            api::paged(Dummy, Pagination::All).iter(&client).collect();

        let err = res.unwrap_err();

        if let ApiError::SpotifyService { status, .. } = err {
            assert_eq!(status, StatusCode::NOT_FOUND);
        } else {
            panic!("unexpected error: {err}");
        }
    }

    #[test]
    fn error_detection() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("paged_dummy")
            .add_query_params(&[("offset", "0"), ("limit", "50")])
            .status(StatusCode::NOT_FOUND)
            .build();

        let client = SingleTestClient::new_json(
            endpoint,
            &json!({
                "message": "dummy error message",
            }),
        );

        let res: Result<Vec<DummyResult>, _> =
            api::paged(Dummy, Pagination::All).iter(&client).collect();

        let err = res.unwrap_err();

        if let ApiError::SpotifyWithStatus { status, msg } = err {
            assert_eq!(status, StatusCode::NOT_FOUND);
            assert_eq!(msg, "dummy error message");
        } else {
            panic!("unexpected error: {err}");
        }
    }
}
