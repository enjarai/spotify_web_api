use crate::api::prelude::*;

/// Get Spotify catalog information for multiple albums identified by their Spotify IDs.
#[derive(Debug, Builder, Clone, Endpoint)]
#[endpoint(method = GET, path = "albums")]
pub struct GetSeveralAlbums {
    /// A list of [Spotify IDs](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the albums.
    ids: Vec<String>,

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

impl GetSeveralAlbumsBuilder {
    pub fn id(&mut self, id: impl Into<String>) -> &mut Self {
        self.ids.get_or_insert_with(Vec::new).push(id.into());
        self
    }
}

impl GetSeveralAlbums {
    pub fn builder() -> GetSeveralAlbumsBuilder {
        GetSeveralAlbumsBuilder::default()
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
    fn test_get_several_albums_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("albums")
            .add_query_params(&[(
                "ids",
                "382ObEPsp2rxGrnsizN5TX,1A2GTWGtFfWp7KSQTwWOyo,2noRn2Aes5aoNVsU6iWThc",
            )])
            .build()
            .unwrap();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GetSeveralAlbums::builder()
            .id("382ObEPsp2rxGrnsizN5TX")
            .id("1A2GTWGtFfWp7KSQTwWOyo")
            .id("2noRn2Aes5aoNVsU6iWThc")
            .build()
            .unwrap();

        api::ignore(endpoint).query(&client).unwrap();
    }
}
