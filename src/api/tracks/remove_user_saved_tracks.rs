use crate::api::prelude::*;

/// Remove one or more tracks from the current user's library.
///
/// This API endpoint is in beta and could change without warning. Please share any feedback that you have, or issues that you discover, in the [Spotify developer community forum](https://community.spotify.com/t5/Spotify-for-Developers/bd-p/Spotify_Developer).
#[derive(Debug, Clone, Endpoint)]
#[endpoint(method = DELETE, path = "me/tracks")]
pub struct RemoveUserSavedTracks {
    /// A list of [Spotify IDs](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the tracks.
    pub ids: Vec<String>,
}

impl<T, I> From<I> for RemoveUserSavedTracks
where
    I: IntoIterator<Item = T>,
    T: Into<String>,
{
    fn from(ids: I) -> Self {
        Self {
            ids: ids.into_iter().map(Into::into).collect(),
        }
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
    fn test_remove_user_saved_tracks_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::DELETE)
            .endpoint("me/tracks")
            .add_query_params(&[("ids", "39joRyXYyjSpI6nKZHyWmH,5mPY98zmeNSp8cmrRtdUW3")])
            .build();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint =
            RemoveUserSavedTracks::from(["39joRyXYyjSpI6nKZHyWmH", "5mPY98zmeNSp8cmrRtdUW3"]);

        api::ignore(endpoint).query(&client).unwrap();
    }
}
