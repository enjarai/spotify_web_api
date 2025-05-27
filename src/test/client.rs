use crate::{
    api::{ApiError, AsyncClient, Client, RestClient},
    model::Page,
};
use async_trait::async_trait;
use bytes::Bytes;
use http::{Method, Response, StatusCode, header, request::Builder as RequestBuilder};
use serde::Serialize;
use std::{borrow::Cow, collections::HashMap, fmt::Debug};
use thiserror::Error;
use url::Url;

#[derive(Debug)]
pub struct ExpectedUrl {
    // #[builder(default = "Method::GET")]
    pub method: Method,

    pub endpoint: &'static str,

    // #[builder(default)]
    pub query: Vec<(Cow<'static, str>, Cow<'static, str>)>,

    // #[builder(setter(strip_option, into), default)]
    pub content_type: Option<String>,

    // #[builder(default)]
    pub body: Vec<u8>,

    // #[builder(default = "StatusCode::OK")]
    pub status: StatusCode,

    // #[builder(default = "false")]
    pub paginated: bool,
}

impl ExpectedUrl {
    pub fn builder() -> ExpectedUrlBuilder {
        ExpectedUrlBuilder::default()
    }

    fn check(&self, method: &Method, url: &Url) {
        // test that the method is as expected.
        assert_eq!(method, self.method);

        // ensure that the URL was not tampered with in the meantime.
        assert_eq!(url.scheme(), "https");
        assert_eq!(url.username(), "");
        assert_eq!(url.password(), None);
        assert_eq!(url.host_str().unwrap(), "api.spotify.com");
        assert_eq!(url.port(), None);
        assert_eq!(url.path(), format!("/v1/{}", self.endpoint));

        let mut count = 0;

        url.query_pairs().into_iter().for_each(|(key, value)| {
            if self.paginated && Self::is_pagination_key(&key) {
                return;
            }

            let found = self.query.iter().any(|(expected_key, expected_value)| {
                &key == expected_key && &value == expected_value
            });
            assert!(found, "unexpected query parameter `{key}={value}`");
            count += 1;
        });

        assert_eq!(count, self.query.len());
        assert_eq!(url.fragment(), None);
    }

    #[inline(always)]
    fn is_pagination_key(key: &str) -> bool {
        matches!(key, "limit" | "offset")
    }
}

pub struct ExpectedUrlBuilder {
    method: Method,
    endpoint: &'static str,
    query: Option<Vec<(Cow<'static, str>, Cow<'static, str>)>>,
    content_type: Option<String>,
    body: Option<Vec<u8>>,
    status: StatusCode,
    paginated: bool,
}

impl ExpectedUrlBuilder {
    pub fn method(&mut self, method: Method) -> &mut Self {
        self.method = method;
        self
    }

    pub fn endpoint(&mut self, endpoint: &'static str) -> &mut Self {
        self.endpoint = endpoint;
        self
    }

    pub fn add_query_params(&mut self, pairs: &[(&'static str, &'static str)]) -> &mut Self {
        self.query
            .get_or_insert_with(Vec::new)
            .extend(pairs.iter().copied().map(|(k, v)| (k.into(), v.into())));
        self
    }

    pub fn content_type(&mut self, content_type: impl Into<String>) -> &mut Self {
        self.content_type = Some(content_type.into());
        self
    }

    pub fn body_str(&mut self, body: &str) -> &mut Self {
        self.body = Some(body.bytes().collect());
        self
    }

    pub fn status(&mut self, status: StatusCode) -> &mut Self {
        self.status = status;
        self
    }

    pub fn paginated(&mut self, paginated: bool) -> &mut Self {
        self.paginated = paginated;
        self
    }

    pub fn build(&self) -> ExpectedUrl {
        ExpectedUrl {
            method: self.method.clone(),
            endpoint: self.endpoint,
            query: self.query.clone().unwrap_or_default(),
            content_type: self.content_type.clone(),
            body: self.body.clone().unwrap_or_default(),
            status: self.status,
            paginated: self.paginated,
        }
    }
}

impl Default for ExpectedUrlBuilder {
    fn default() -> Self {
        Self {
            method: Method::GET,
            endpoint: "",
            query: None,
            content_type: None,
            body: None,
            status: StatusCode::OK,
            paginated: false,
        }
    }
}

#[derive(Debug, Clone)]
struct MockResponse {
    status: StatusCode,
    data: Vec<u8>,
}

impl MockResponse {
    fn response(&self) -> Response<Vec<u8>> {
        Response::builder()
            .status(self.status)
            .body(self.data.clone())
            .unwrap()
    }
}

#[derive(Debug, Default)]
struct MockClient {
    response_map: HashMap<(Method, String), MockResponse>,
}

pub struct SingleTestClient {
    client: MockClient,
    expected: ExpectedUrl,
}

impl SingleTestClient {
    pub fn new_raw<T>(expected: ExpectedUrl, data: T) -> Self
    where
        T: Into<Vec<u8>>,
    {
        let mut client = MockClient::default();

        let request = (
            expected.method.clone(),
            format!("/v1/{}", expected.endpoint),
        );
        let response = MockResponse {
            status: expected.status,
            data: data.into(),
        };

        client.response_map.insert(request, response);

        Self { client, expected }
    }

    pub fn new_json<T>(expected: ExpectedUrl, data: &T) -> Self
    where
        T: Serialize,
    {
        let data = serde_json::to_vec(data).unwrap();
        Self::new_raw(expected, data)
    }
}

#[derive(Debug, Error)]
#[error("test client error")]
pub enum TestClientError {}

impl RestClient for SingleTestClient {
    type Error = TestClientError;

    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, ApiError<Self::Error>> {
        Ok(Url::parse(&format!(
            "https://api.spotify.com/v1/{endpoint}"
        ))?)
    }
}

impl Client for SingleTestClient {
    fn rest(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>> {
        let url = Url::parse(&format!("{}", request.uri_ref().unwrap())).unwrap();
        self.expected
            .check(&request.method_ref().unwrap().clone(), &url);

        assert_eq!(
            &body,
            &self.expected.body,
            "\nbody is not the same:\nactual  : {}\nexpected: {}\n",
            String::from_utf8_lossy(&body),
            String::from_utf8_lossy(&self.expected.body),
        );

        let headers = request.headers_ref().unwrap();
        let content_type = headers
            .get_all(header::CONTENT_TYPE)
            .iter()
            .map(|value| value.to_str().unwrap());

        if let Some(expected_content_type) = self.expected.content_type.as_ref() {
            itertools::assert_equal(
                content_type,
                std::iter::once(&expected_content_type).copied(),
            );
        } else {
            assert_eq!(content_type.count(), 0);
        }

        let request = request.body(body).unwrap();

        Ok(self
            .client
            .response_map
            .get(&(request.method().clone(), request.uri().path().into()))
            .expect("no matching request found")
            .response()
            .map(Into::into))
    }
}

#[async_trait]
impl AsyncClient for SingleTestClient {
    async fn rest_async(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<<Self as RestClient>::Error>> {
        <Self as Client>::rest(self, request, body)
    }
}

const DEFAULT_LIMIT: usize = 20;

pub struct PagedTestClient<T> {
    expected: ExpectedUrl,
    data: Vec<T>,
}

impl<T> PagedTestClient<T> {
    pub fn new_raw<I>(expected: ExpectedUrl, data: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self {
            expected,
            data: data.into_iter().collect(),
        }
    }
}

impl<T> RestClient for PagedTestClient<T> {
    type Error = TestClientError;

    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, ApiError<Self::Error>> {
        Ok(Url::parse(&format!(
            "https://api.spotify.com/v1/{endpoint}"
        ))?)
    }
}

impl<T> Client for PagedTestClient<T>
where
    T: Debug + Clone + Serialize,
{
    fn rest(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>> {
        let url = Url::parse(&format!("{}", request.uri_ref().unwrap())).unwrap();

        self.expected.check(request.method_ref().unwrap(), &url);

        assert_eq!(
            &body,
            &self.expected.body,
            "\nbody is not the same:\nactual  : {}\nexpected: {}\n",
            String::from_utf8_lossy(&body),
            String::from_utf8_lossy(&self.expected.body),
        );

        let headers = request.headers_ref().unwrap();

        let content_type = headers
            .get_all(header::CONTENT_TYPE)
            .iter()
            .map(|value| value.to_str().unwrap());

        if let Some(expected_content_type) = self.expected.content_type.as_ref() {
            itertools::assert_equal(
                content_type,
                std::iter::once(&expected_content_type).copied(),
            );
        } else {
            assert_eq!(content_type.count(), 0);
        }

        let mut offset: usize = 0;
        let mut limit = DEFAULT_LIMIT;

        url.query_pairs()
            .into_iter()
            .for_each(|(key, value)| match key.as_ref() {
                "offset" => {
                    offset = value.parse().unwrap();
                }
                "limit" => {
                    limit = value.parse().unwrap();
                }
                _ => (),
            });

        let range = {
            let mut range = offset..offset + limit;
            range.end = std::cmp::min(range.end, self.data.len());
            range
        };

        let request = request.body(body).unwrap();

        assert_eq!(*request.method(), Method::GET);

        let previous = if offset > 0 {
            let previous_offset = if limit > offset {
                0
            } else {
                offset.saturating_sub(limit)
            };
            Some(
                url.clone()
                    .query_pairs_mut()
                    .clear()
                    .append_pair("offset", previous_offset.to_string().as_str())
                    .append_pair("limit", limit.to_string().as_str())
                    .finish()
                    .to_string(),
            )
        } else {
            None
        };

        let next = if range.end < self.data.len() {
            Some(
                url.clone()
                    .query_pairs_mut()
                    .clear()
                    .append_pair("offset", range.end.to_string().as_str())
                    .append_pair("limit", limit.to_string().as_str())
                    .finish()
                    .to_string(),
            )
        } else {
            None
        };

        let page = Page {
            href: url.as_str().to_owned(),
            limit,
            next,
            offset,
            previous,
            total: self.data.len(),
            items: self.data[range].to_vec(),
        };

        let response = Response::builder()
            .status(self.expected.status)
            .body(serde_json::to_vec(&page).unwrap())
            .unwrap()
            .map(Into::into);

        Ok(response)
    }
}

#[async_trait]
impl<T> AsyncClient for PagedTestClient<T>
where
    T: Debug + Clone + Serialize + Send + Sync,
{
    async fn rest_async(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<<Self as RestClient>::Error>> {
        <Self as Client>::rest(self, request, body)
    }
}
