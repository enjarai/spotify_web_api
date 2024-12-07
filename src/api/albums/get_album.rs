use crate::api::prelude::*;

/// Get Spotify catalog information for a single album.
#[derive(Debug, Builder, Clone, Endpoint)]
#[endpoint(method = GET, path = "albums/{id}")]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        api::{self, Query as _},
        test::client::{ExpectedUrl, SingleTestClient},
    };

    #[test]
    fn test_get_album_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("albums/0oKvU088cLhKbbVvQc9lQF")
            .build()
            .unwrap();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GetAlbum::from("0oKvU088cLhKbbVvQc9lQF");

        api::ignore(endpoint).query(&client).unwrap();
    }
}
