use crate::api::prelude::*;

/// Add the current user as a follower of a playlist.
#[derive(Debug, Clone)]
pub struct FollowPlaylist {
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) of the playlist.
    pub id: String,

    /// Defaults to true.
    /// If true the playlist will be included in user's public playlists (added to profile),
    /// if false it will remain private.
    /// For more about public/private status, see [Working with Playlists](https://developer.spotify.com/documentation/web-api/concepts/playlists)
    pub public: bool,
}

impl FollowPlaylist {
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

impl Endpoint for FollowPlaylist {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("playlists/{}/followers", self.id).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        JsonParams::into_body(&serde_json::json!({
            "public": self.public,
        }))
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
            .build();

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
            .build();

        let client = SingleTestClient::new_raw(endpoint, "");

        api::ignore(FollowPlaylist::new("3cEYpjA9oz9GiPac4AsH4n", false))
            .query(&client)
            .unwrap();
    }
}
