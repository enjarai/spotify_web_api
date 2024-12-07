use crate::api::prelude::*;

/// Get Spotify catalog information about an artist's top tracks by country.
#[derive(Debug, Builder, Clone, Endpoint)]
#[endpoint(method = GET, path = "artists/{id}/top-tracks")]
pub struct GetArtistTopTracks {
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the artist.
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

impl GetArtistTopTracks {
    pub fn builder() -> GetArtistTopTracksBuilder {
        GetArtistTopTracksBuilder::default()
    }
}

impl From<&str> for GetArtistTopTracks {
    fn from(id: &str) -> Self {
        Self {
            id: id.to_owned(),
            market: None,
        }
    }
}

impl From<String> for GetArtistTopTracks {
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
            .endpoint("artists/0TnOYISbd1XYRBk9myaseg/top-tracks")
            .build()
            .unwrap();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GetArtistTopTracks::from("0TnOYISbd1XYRBk9myaseg");

        api::ignore(endpoint).query(&client).unwrap();
    }
}
