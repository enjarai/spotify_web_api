use crate::api::Endpoint;
use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

/// Get Spotify catalog information for a single artist identified by their unique Spotify ID.
#[derive(Debug, Builder, Clone)]
pub struct ArtistEndpoint {
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) of the artist.
    #[builder(setter(into))]
    id: String,
}

impl ArtistEndpoint {
    pub fn builder() -> ArtistEndpointBuilder {
        ArtistEndpointBuilder::default()
    }
}

impl Endpoint for ArtistEndpoint {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("artists/{}", self.id).into()
    }
}

impl From<&str> for ArtistEndpoint {
    fn from(id: &str) -> Self {
        Self { id: id.to_owned() }
    }
}

impl From<String> for ArtistEndpoint {
    fn from(id: String) -> Self {
        Self { id }
    }
}

// #[builder(setter(into, strip_option), default)]
// market: Option<Market>,
