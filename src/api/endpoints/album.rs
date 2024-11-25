use crate::api::Endpoint;
use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

/// Get Spotify catalog information for a single album.
#[derive(Debug, Builder, Clone)]
pub struct AlbumEndpoint {
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) of the album.
    #[builder(setter(into))]
    id: String,
}

impl AlbumEndpoint {
    pub fn builder() -> AlbumEndpointBuilder {
        AlbumEndpointBuilder::default()
    }
}

impl Endpoint for AlbumEndpoint {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("albums/{}", self.id).into()
    }
}

impl From<&str> for AlbumEndpoint {
    fn from(id: &str) -> Self {
        Self { id: id.to_owned() }
    }
}

impl From<String> for AlbumEndpoint {
    fn from(id: String) -> Self {
        Self { id }
    }
}
