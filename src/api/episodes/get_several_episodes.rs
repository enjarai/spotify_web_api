use crate::api::prelude::*;

/// Get Spotify catalog information for several episodes based on their Spotify IDs.
#[derive(Debug, Builder, Clone, Endpoint)]
#[endpoint(method = GET, path = "episodes")]
pub struct GetSeveralEpisodes {
    /// A list of [Spotify IDs](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the episodes.
    pub ids: Vec<String>,

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

impl GetSeveralEpisodesBuilder {
    pub fn id(&mut self, id: impl Into<String>) -> &mut Self {
        self.ids.get_or_insert_with(Vec::new).push(id.into());
        self
    }
}

impl GetSeveralEpisodes {
    pub fn builder() -> GetSeveralEpisodesBuilder {
        GetSeveralEpisodesBuilder::default()
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
    fn test_get_several_episodes_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("episodes")
            .add_query_params(&[("ids", "77o6BIVlYM3msb4MMIL1jH,0Q86acNRm6V9GYx55SXKwf")])
            .build()
            .unwrap();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GetSeveralEpisodes::builder()
            .id("77o6BIVlYM3msb4MMIL1jH")
            .id("0Q86acNRm6V9GYx55SXKwf")
            .build()
            .unwrap();

        api::ignore(endpoint).query(&client).unwrap();
    }
}
