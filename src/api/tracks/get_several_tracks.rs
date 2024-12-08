use crate::api::prelude::*;

/// Get Spotify catalog information for several tracks based on their Spotify IDs.
#[derive(Debug, Builder, Clone, Endpoint)]
#[endpoint(method = GET, path = "tracks")]
pub struct GetSeveralTracks {
    /// A list of [Spotify IDs](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the tracks.
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

#[allow(dead_code)]
impl GetSeveralTracksBuilder {
    fn id(&mut self, id: impl Into<String>) -> &mut Self {
        self.ids.get_or_insert_with(Vec::new).push(id.into());
        self
    }
}

impl GetSeveralTracks {
    pub fn builder() -> GetSeveralTracksBuilder {
        GetSeveralTracksBuilder::default()
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
    fn test_get_several_tracks_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("tracks")
            .add_query_params(&[("ids", "39joRyXYyjSpI6nKZHyWmH,5mPY98zmeNSp8cmrRtdUW3")])
            .build()
            .unwrap();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GetSeveralTracks::builder()
            .id("39joRyXYyjSpI6nKZHyWmH")
            .id("5mPY98zmeNSp8cmrRtdUW3")
            .build()
            .unwrap();

        api::ignore(endpoint).query(&client).unwrap();
    }
}
