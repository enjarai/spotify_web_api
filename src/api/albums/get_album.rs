use crate::api::Endpoint;
use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

/// Get Spotify catalog information for a single album.
#[derive(Debug, Builder, Clone)]
pub struct GetAlbum {
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) of the album.
    #[builder(setter(into))]
    id: String,
}

impl GetAlbum {
    pub fn builder() -> GetAlbumBuilder {
        GetAlbumBuilder::default()
    }
}

impl Endpoint for GetAlbum {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("albums/{}", self.id).into()
    }
}

impl From<&str> for GetAlbum {
    fn from(id: &str) -> Self {
        Self { id: id.to_owned() }
    }
}

impl From<String> for GetAlbum {
    fn from(id: String) -> Self {
        Self { id }
    }
}
