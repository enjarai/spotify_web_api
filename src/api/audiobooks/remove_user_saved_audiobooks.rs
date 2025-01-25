use crate::api::prelude::*;

/// Remove one or more audiobooks from the Spotify user's library.
#[derive(Debug, Builder, Clone, Endpoint)]
#[endpoint(method = DELETE, path = "me/audiobooks")]
pub struct RemoveUserSavedAudiobooks {
    /// A list of [Spotify IDs](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the audiobooks.
    pub ids: Vec<String>,
}

impl RemoveUserSavedAudiobooksBuilder {
    pub fn id(&mut self, id: impl Into<String>) -> &mut Self {
        self.ids.get_or_insert_with(Vec::new).push(id.into());
        self
    }
}

impl RemoveUserSavedAudiobooks {
    pub fn builder() -> RemoveUserSavedAudiobooksBuilder {
        RemoveUserSavedAudiobooksBuilder::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        api::{self, Query as _},
        test::client::{ExpectedUrl, SingleTestClient},
    };
    use http::Method;

    #[test]
    fn test_remove_user_saved_audiobooks_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::DELETE)
            .endpoint("me/audiobooks")
            .add_query_params(&[(
                "ids",
                "18yVqkdbdRvS24c0Ilj2ci,1HGw3J3NxZO1TP1BTtVhpZ,7iHfbu1YPACw6oZPAFJtqe",
            )])
            .build()
            .unwrap();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = RemoveUserSavedAudiobooks::builder()
            .id("18yVqkdbdRvS24c0Ilj2ci")
            .id("1HGw3J3NxZO1TP1BTtVhpZ")
            .id("7iHfbu1YPACw6oZPAFJtqe")
            .build()
            .unwrap();

        api::ignore(endpoint).query(&client).unwrap();
    }
}
