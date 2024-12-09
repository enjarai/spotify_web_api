use crate::api::prelude::*;

/// Save one or more tracks to the current user's library.
///
/// This API endpoint is in beta and could change without warning. Please share any feedback that you have, or issues that you discover, in the [Spotify developer community forum](https://community.spotify.com/t5/Spotify-for-Developers/bd-p/Spotify_Developer).
#[derive(Debug, Builder, Clone, Endpoint)]
#[endpoint(method = PUT, path = "me/tracks")]
pub struct SaveTracksforCurrentUser {
    /// A list of [Spotify IDs](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the tracks.
    ids: Vec<String>,
}

impl SaveTracksforCurrentUserBuilder {
    pub fn id(&mut self, id: impl Into<String>) -> &mut Self {
        self.ids.get_or_insert_with(Vec::new).push(id.into());
        self
    }
}

impl SaveTracksforCurrentUser {
    pub fn builder() -> SaveTracksforCurrentUserBuilder {
        SaveTracksforCurrentUserBuilder::default()
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
    fn test_save_tracks_for_current_user_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("me/tracks")
            .add_query_params(&[("ids", "39joRyXYyjSpI6nKZHyWmH,5mPY98zmeNSp8cmrRtdUW3")])
            .build()
            .unwrap();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = SaveTracksforCurrentUser::builder()
            .id("39joRyXYyjSpI6nKZHyWmH")
            .id("5mPY98zmeNSp8cmrRtdUW3")
            .build()
            .unwrap();

        api::ignore(endpoint).query(&client).unwrap();
    }
}
