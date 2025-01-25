use crate::api::prelude::*;

/// Get Spotify catalog information for a single album.
#[derive(Debug, Clone, Endpoint)]
#[endpoint(method = GET, path = "albums/{id}")]
pub struct GetAlbum {
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) of the album.
    pub id: String,
}

impl GetAlbum {
    pub fn new(id: impl Into<String>) -> Self {
        Self::from(id)
    }
}

impl<T: Into<String>> From<T> for GetAlbum {
    fn from(id: T) -> Self {
        Self { id: id.into() }
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

        let endpoint = GetAlbum::new("0oKvU088cLhKbbVvQc9lQF");

        api::ignore(endpoint).query(&client).unwrap();
    }
}
