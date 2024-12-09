use crate::api::prelude::*;

/// Get Spotify catalog information for several shows based on their Spotify IDs.
#[derive(Debug, Builder, Clone, Endpoint)]
#[endpoint(method = GET, path = "shows")]
pub struct GetSeveralShows {
    /// A list of [Spotify IDs](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the shows.
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

impl GetSeveralShowsBuilder {
    pub fn id(&mut self, id: impl Into<String>) -> &mut Self {
        self.ids.get_or_insert_with(Vec::new).push(id.into());
        self
    }
}

impl GetSeveralShows {
    pub fn builder() -> GetSeveralShowsBuilder {
        GetSeveralShowsBuilder::default()
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
    fn test_get_several_shows_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("shows")
            .add_query_params(&[("ids", "5CfCWKI5pZ28U0uOzXkDHe,5as3aKmN2k11yfDDDSrvaZ")])
            .build()
            .unwrap();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GetSeveralShows::builder()
            .id("5CfCWKI5pZ28U0uOzXkDHe")
            .id("5as3aKmN2k11yfDDDSrvaZ")
            .build()
            .unwrap();

        api::ignore(endpoint).query(&client).unwrap();
    }
}
