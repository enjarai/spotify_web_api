use crate::api::prelude::*;

/// Add the current user as a follower of a playlist.
#[derive(Debug, Builder, Clone, Endpoint)]
#[endpoint(method = PUT, path = "playlists/{id}/followers")]
pub struct FollowPlaylist {
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) of the playlist.
    pub id: String,

    /// Defaults to true.
    /// If true the playlist will be included in user's public playlists (added to profile),
    /// if false it will remain private.
    /// For more about public/private status, see [Working with Playlists](https://developer.spotify.com/documentation/web-api/concepts/playlists)
    #[builder(default = true)]
    #[endpoint(body)]
    pub public: bool,
}

impl FollowPlaylist {
    pub fn builder() -> FollowPlaylistBuilder {
        FollowPlaylistBuilder::default()
    }

    pub fn new(id: impl Into<String>, public: bool) -> Self {
        Self {
            id: id.into(),
            public,
        }
    }
}

impl<T: Into<String>> From<T> for FollowPlaylist {
    fn from(id: T) -> Self {
        Self {
            id: id.into(),
            public: true,
        }
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
    fn test_follow_playlist_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("playlists/3cEYpjA9oz9GiPac4AsH4n/followers")
            .method(Method::PUT)
            .content_type("application/json")
            .body_str(r#"{"public":true}"#)
            .build()
            .unwrap();

        let client = SingleTestClient::new_raw(endpoint, "");

        api::ignore(FollowPlaylist::from("3cEYpjA9oz9GiPac4AsH4n"))
            .query(&client)
            .unwrap();
    }

    #[test]
    fn test_follow_playlist_endpoint_with_public_false() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("playlists/3cEYpjA9oz9GiPac4AsH4n/followers")
            .method(Method::PUT)
            .content_type("application/json")
            .body_str(r#"{"public":false}"#)
            .build()
            .unwrap();

        let client = SingleTestClient::new_raw(endpoint, "");

        api::ignore(FollowPlaylist::new("3cEYpjA9oz9GiPac4AsH4n", false))
            .query(&client)
            .unwrap();
    }
}
