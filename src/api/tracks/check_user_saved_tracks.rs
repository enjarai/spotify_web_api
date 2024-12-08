use crate::api::prelude::*;

/// Check if one or more tracks are already saved in the current Spotify user's library.
#[derive(Debug, Builder, Clone, Endpoint)]
#[endpoint(method = GET, path = "me/tracks/contains")]
pub struct CheckUserSavedTracks {
    /// A list of [Spotify IDs](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the tracks.
    ids: Vec<String>,
}

#[allow(dead_code)]
impl CheckUserSavedTracksBuilder {
    fn id(&mut self, id: impl Into<String>) -> &mut Self {
        self.ids.get_or_insert_with(Vec::new).push(id.into());
        self
    }
}

impl CheckUserSavedTracks {
    pub fn builder() -> CheckUserSavedTracksBuilder {
        CheckUserSavedTracksBuilder::default()
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
    fn test_check_user_saved_tracks_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("me/tracks/contains")
            .add_query_params(&[("ids", "39joRyXYyjSpI6nKZHyWmH,5mPY98zmeNSp8cmrRtdUW3")])
            .build()
            .unwrap();

        let expected_response = [false, false];

        let client = SingleTestClient::new_json(endpoint, &expected_response);

        let endpoint = CheckUserSavedTracks::builder()
            .id("39joRyXYyjSpI6nKZHyWmH")
            .id("5mPY98zmeNSp8cmrRtdUW3")
            .build()
            .unwrap();

        let result: Vec<bool> = endpoint.query(&client).unwrap();

        assert_eq!(result, expected_response);
    }
}
