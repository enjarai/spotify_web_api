use crate::api::prelude::*;

/// Check if one or more shows are already saved in the current Spotify user's library.
#[derive(Debug, Builder, Clone, Endpoint)]
#[endpoint(method = GET, path = "me/shows/contains")]
pub struct CheckUserSavedShows {
    /// A list of [Spotify IDs](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the shows.
    ids: Vec<String>,
}

impl CheckUserSavedShowsBuilder {
    pub fn id(&mut self, id: impl Into<String>) -> &mut Self {
        self.ids.get_or_insert_with(Vec::new).push(id.into());
        self
    }
}

impl CheckUserSavedShows {
    pub fn builder() -> CheckUserSavedShowsBuilder {
        CheckUserSavedShowsBuilder::default()
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
    fn test_check_user_saved_shows_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("me/shows/contains")
            .add_query_params(&[("ids", "5CfCWKI5pZ28U0uOzXkDHe,5as3aKmN2k11yfDDDSrvaZ")])
            .build()
            .unwrap();

        let expected_response = [false, false];

        let client = SingleTestClient::new_json(endpoint, &expected_response);

        let endpoint = CheckUserSavedShows::builder()
            .id("5CfCWKI5pZ28U0uOzXkDHe")
            .id("5as3aKmN2k11yfDDDSrvaZ")
            .build()
            .unwrap();

        let result: Vec<bool> = endpoint.query(&client).unwrap();

        assert_eq!(result, expected_response);
    }
}
