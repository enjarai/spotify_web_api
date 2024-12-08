use crate::api::prelude::*;

/// Save one or more episodes to the current user's library.
///
/// This API endpoint is in beta and could change without warning. Please share any feedback that you have, or issues that you discover, in the [Spotify developer community forum](https://community.spotify.com/t5/Spotify-for-Developers/bd-p/Spotify_Developer).
#[derive(Debug, Builder, Clone, Endpoint)]
#[endpoint(method = PUT, path = "me/episodes")]
pub struct SaveEpisodesforCurrentUser {
    /// A list of [Spotify IDs](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the episodes.
    #[endpoint(body)]
    ids: Vec<String>,
}

#[allow(dead_code)]
impl SaveEpisodesforCurrentUserBuilder {
    fn id(&mut self, id: impl Into<String>) -> &mut Self {
        self.ids.get_or_insert_with(Vec::new).push(id.into());
        self
    }
}

impl SaveEpisodesforCurrentUser {
    pub fn builder() -> SaveEpisodesforCurrentUserBuilder {
        SaveEpisodesforCurrentUserBuilder::default()
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
    fn test_save_episodes_for_current_user_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("me/episodes")
            .content_type("application/json")
            .body_str(r#"{"ids":["77o6BIVlYM3msb4MMIL1jH","0Q86acNRm6V9GYx55SXKwf"]}"#)
            .build()
            .unwrap();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = SaveEpisodesforCurrentUser::builder()
            .id("77o6BIVlYM3msb4MMIL1jH")
            .id("0Q86acNRm6V9GYx55SXKwf")
            .build()
            .unwrap();

        api::ignore(endpoint).query(&client).unwrap();
    }
}
