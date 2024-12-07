use crate::api::prelude::*;

/// Check to see if the current user is following a specified playlist.
#[derive(Debug, Clone, Endpoint)]
#[endpoint(method = GET, path = "playlists/{id}/followers/contains")]
pub struct CheckIfCurrentUserFollowsPlaylist {
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) of the playlist.
    id: String,
}

impl CheckIfCurrentUserFollowsPlaylist {
    pub fn new(id: impl Into<String>) -> Self {
        Self::from(id)
    }
}

impl<T: Into<String>> From<T> for CheckIfCurrentUserFollowsPlaylist {
    fn from(id: T) -> Self {
        Self { id: id.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        api::Query as _,
        test::client::{ExpectedUrl, SingleTestClient},
    };

    #[test]
    fn test_check_if_current_user_follows_playlist_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("playlists/3cEYpjA9oz9GiPac4AsH4n/followers/contains")
            .build()
            .unwrap();

        let expected_response = [false];

        let client = SingleTestClient::new_json(endpoint, &expected_response);

        let endpoint = CheckIfCurrentUserFollowsPlaylist::new("3cEYpjA9oz9GiPac4AsH4n");

        let result: Vec<bool> = endpoint.query(&client).unwrap();

        assert_eq!(result, expected_response);
    }
}
