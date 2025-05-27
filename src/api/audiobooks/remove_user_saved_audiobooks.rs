use crate::api::prelude::*;

/// Remove one or more audiobooks from the Spotify user's library.
#[derive(Debug, Clone, Endpoint)]
#[endpoint(method = DELETE, path = "me/audiobooks")]
pub struct RemoveUserSavedAudiobooks {
    /// A list of [Spotify IDs](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the audiobooks.
    pub ids: Vec<String>,
}

impl<T, I> From<I> for RemoveUserSavedAudiobooks
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
    fn test_remove_user_saved_audiobooks_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::DELETE)
            .endpoint("me/audiobooks")
            .add_query_params(&[(
                "ids",
                "18yVqkdbdRvS24c0Ilj2ci,1HGw3J3NxZO1TP1BTtVhpZ,7iHfbu1YPACw6oZPAFJtqe",
            )])
            .build();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = RemoveUserSavedAudiobooks::from([
            "18yVqkdbdRvS24c0Ilj2ci",
            "1HGw3J3NxZO1TP1BTtVhpZ",
            "7iHfbu1YPACw6oZPAFJtqe",
        ]);

        api::ignore(endpoint).query(&client).unwrap();
    }
}
