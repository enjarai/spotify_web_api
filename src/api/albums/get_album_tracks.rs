use crate::api::prelude::*;

/// Get Spotify catalog information about an album’s tracks. Optional parameters can be used to limit the number of tracks returned.
#[derive(Debug, Builder, Clone, Endpoint)]
#[endpoint(method = GET, path = "albums/{id}/tracks")]
pub struct GetAlbumTracks {
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) of the album.
    #[builder(setter(into))]
    id: String,

    /// An [ISO 3166-1 alpha-2 country code](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2).
    /// If a country code is specified, only content that is available in that market will be returned.
    /// If a valid user access token is specified in the request header, the country associated with the user account will take priority over this parameter.
    ///
    /// # Notes
    /// If neither market or user country are provided, the content is considered unavailable for the client.
    /// Users can view the country that is associated with their account in the [account settings](https://www.spotify.com/account/overview/).
    #[builder(setter(into, strip_option), default)]
    market: Option<Market>,
}

impl GetAlbumTracks {
    pub fn builder() -> GetAlbumTracksBuilder {
        GetAlbumTracksBuilder::default()
    }
}

impl Pageable for GetAlbumTracks {}

impl From<&str> for GetAlbumTracks {
    fn from(id: &str) -> Self {
        Self {
            id: id.to_owned(),
            market: None,
        }
    }
}

impl From<String> for GetAlbumTracks {
    fn from(id: String) -> Self {
        Self { id, market: None }
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
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("albums/66q0q162WOFuidbucN1Jdp/tracks")
            .build()
            .unwrap();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GetAlbumTracks::from("66q0q162WOFuidbucN1Jdp");

        api::ignore(endpoint).query(&client).unwrap();
    }
}
