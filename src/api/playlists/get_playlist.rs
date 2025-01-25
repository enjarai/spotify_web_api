use crate::api::prelude::*;

/// Get a playlist owned by a Spotify user.
#[derive(Debug, Builder, Clone, Endpoint)]
#[endpoint(method = GET, path = "playlists/{id}")]
pub struct GetPlaylist {
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) of the playlist.
    #[builder(setter(into))]
    pub id: String,

    /// An [ISO 3166-1 alpha-2 country code](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2).
    /// If a country code is specified, only content that is available in that market will be returned.
    /// If a valid user access token is specified in the request header, the country associated with the user account will take priority over this parameter.
    ///
    /// # Notes
    /// If neither market or user country are provided, the content is considered unavailable for the client.
    /// Users can view the country that is associated with their account in the [account settings](https://www.spotify.com/account/overview/).
    #[builder(setter(into, strip_option), default)]
    pub market: Option<Market>,
}

impl GetPlaylist {
    pub fn builder() -> GetPlaylistBuilder {
        GetPlaylistBuilder::default()
    }
}

impl<T: Into<String>> From<T> for GetPlaylist {
    fn from(id: T) -> Self {
        Self {
            id: id.into(),
            market: None,
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
    fn test_get_playlist_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("playlists/3cEYpjA9oz9GiPac4AsH4n")
            .build()
            .unwrap();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GetPlaylist::from("3cEYpjA9oz9GiPac4AsH4n");

        api::ignore(endpoint).query(&client).unwrap();
    }
}
