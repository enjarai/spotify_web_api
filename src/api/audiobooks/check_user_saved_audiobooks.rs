use crate::api::prelude::*;

/// Check if one or more audiobooks are already saved in the current Spotify user's library.
#[derive(Debug, Builder, Clone, Endpoint)]
#[endpoint(method = GET, path = "me/audiobooks/contains")]
pub struct CheckUserSavedAudiobooks {
    /// A list of [Spotify IDs](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the audiobooks.
    ids: Vec<String>,
}

impl CheckUserSavedAudiobooksBuilder {
    pub fn id(&mut self, id: impl Into<String>) -> &mut Self {
        self.ids.get_or_insert_with(Vec::new).push(id.into());
        self
    }
}

impl CheckUserSavedAudiobooks {
    pub fn builder() -> CheckUserSavedAudiobooksBuilder {
        CheckUserSavedAudiobooksBuilder::default()
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
    fn test_check_user_saved_audiobooks_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("me/audiobooks/contains")
            .add_query_params(&[(
                "ids",
                "18yVqkdbdRvS24c0Ilj2ci,1HGw3J3NxZO1TP1BTtVhpZ,7iHfbu1YPACw6oZPAFJtqe",
            )])
            .build()
            .unwrap();

        let expected_response = [false, false, false];

        let client = SingleTestClient::new_json(endpoint, &expected_response);

        let endpoint = CheckUserSavedAudiobooks::builder()
            .id("18yVqkdbdRvS24c0Ilj2ci")
            .id("1HGw3J3NxZO1TP1BTtVhpZ")
            .id("7iHfbu1YPACw6oZPAFJtqe")
            .build()
            .unwrap();

        let result: Vec<bool> = endpoint.query(&client).unwrap();

        assert_eq!(result, expected_response);
    }
}
