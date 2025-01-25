use crate::api::prelude::*;

/// Get the current image associated with a specific playlist.
#[derive(Debug, Clone, Endpoint)]
#[endpoint(method = GET, path = "playlists/{id}/images")]
pub struct GetPlaylistCoverImage {
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) of the playlist.
    pub id: String,
}

impl GetPlaylistCoverImage {
    pub fn new(id: impl Into<String>) -> Self {
        Self::from(id)
    }
}

impl<T: Into<String>> From<T> for GetPlaylistCoverImage {
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
    fn test_get_playlist_cover_image_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("playlists/3cEYpjA9oz9GiPac4AsH4n/images")
            .build()
            .unwrap();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GetPlaylistCoverImage::from("3cEYpjA9oz9GiPac4AsH4n");

        api::ignore(endpoint).query(&client).unwrap();
    }
}
