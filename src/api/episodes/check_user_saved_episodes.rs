use crate::api::prelude::*;

/// Check if one or more episodes is already saved in the current Spotify user's 'Your Episodes' library.
///
/// This API endpoint is in beta and could change without warning. Please share any feedback that you have, or issues that you discover, in the [Spotify developer community forum](https://community.spotify.com/t5/Spotify-for-Developers/bd-p/Spotify_Developer).
#[derive(Debug, Builder, Clone, Endpoint)]
#[endpoint(method = GET, path = "me/episodes/contains")]
pub struct CheckUserSavedEpisodes {
    /// A list of [Spotify IDs](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the episodes.
    pub ids: Vec<String>,
}

impl CheckUserSavedEpisodesBuilder {
    pub fn id(&mut self, id: impl Into<String>) -> &mut Self {
        self.ids.get_or_insert_with(Vec::new).push(id.into());
        self
    }
}

impl CheckUserSavedEpisodes {
    pub fn builder() -> CheckUserSavedEpisodesBuilder {
        CheckUserSavedEpisodesBuilder::default()
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
    fn test_check_user_saved_episodes_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("me/episodes/contains")
            .add_query_params(&[("ids", "77o6BIVlYM3msb4MMIL1jH,0Q86acNRm6V9GYx55SXKwf")])
            .build()
            .unwrap();

        let expected_response = [false, true];

        let client = SingleTestClient::new_json(endpoint, &expected_response);

        let endpoint = CheckUserSavedEpisodes::builder()
            .id("77o6BIVlYM3msb4MMIL1jH")
            .id("0Q86acNRm6V9GYx55SXKwf")
            .build()
            .unwrap();

        let result: Vec<bool> = endpoint.query(&client).unwrap();

        assert_eq!(result, expected_response);
    }
}
