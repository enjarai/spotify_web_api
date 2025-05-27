use crate::api::prelude::*;

/// Remove the current user as a follower of a playlist.
#[derive(Debug, Clone, Endpoint)]
#[endpoint(method = DELETE, path = "playlists/{id}/followers")]
pub struct UnfollowPlaylist {
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) of the playlist.
    pub id: String,
}

impl UnfollowPlaylist {
    pub fn new(id: impl Into<String>) -> Self {
        Self::from(id)
    }
}

impl<T: Into<String>> From<T> for UnfollowPlaylist {
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
    fn test_unfollow_playlist_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("playlists/3cEYpjA9oz9GiPac4AsH4n/followers")
            .method(Method::DELETE)
            .build();

        let client = SingleTestClient::new_raw(endpoint, "");

        api::ignore(UnfollowPlaylist::from("3cEYpjA9oz9GiPac4AsH4n"))
            .query(&client)
            .unwrap();
    }
}
