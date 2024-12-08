use crate::api::prelude::*;

/// Remove one or more episodes from the current user's library.
///
/// This API endpoint is in beta and could change without warning. Please share any feedback that you have, or issues that you discover, in the [Spotify developer community forum](https://community.spotify.com/t5/Spotify-for-Developers/bd-p/Spotify_Developer).
#[derive(Debug, Builder, Clone, Endpoint)]
#[endpoint(method = DELETE, path = "me/episodes")]
pub struct RemoveUserSavedEpisodes {
    /// A list of [Spotify IDs](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the episodes.
    #[endpoint(body)]
    ids: Vec<String>,
}

#[allow(dead_code)]
impl RemoveUserSavedEpisodesBuilder {
    fn id(&mut self, id: impl Into<String>) -> &mut Self {
        self.ids.get_or_insert_with(Vec::new).push(id.into());
        self
    }
}

impl RemoveUserSavedEpisodes {
    pub fn builder() -> RemoveUserSavedEpisodesBuilder {
        RemoveUserSavedEpisodesBuilder::default()
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
    fn test_remove_user_saved_episodes_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::DELETE)
            .endpoint("me/episodes")
            .content_type("application/json")
            .body_str(r#"{"ids":["7ouMYWpwJ422jRcDASZB7P","4VqPOruhp5EdPBeR92t6lQ","2takcwOaAZWiXQijPHIx7B"]}"#)
            .build()
            .unwrap();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = RemoveUserSavedEpisodes::builder()
            .id("7ouMYWpwJ422jRcDASZB7P")
            .id("4VqPOruhp5EdPBeR92t6lQ")
            .id("2takcwOaAZWiXQijPHIx7B")
            .build()
            .unwrap();

        api::ignore(endpoint).query(&client).unwrap();
    }
}
