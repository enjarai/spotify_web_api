use crate::api::prelude::*;

/// Check if one or more albums is already saved in the current Spotify user's 'Your Music' library.
#[derive(Debug, Clone, Endpoint)]
#[endpoint(method = GET, path = "me/albums/contains")]
pub struct CheckUserSavedAlbums {
    /// A list of [Spotify IDs](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the albums.
    pub ids: Vec<String>,
}

impl<T, I> From<I> for CheckUserSavedAlbums
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
        api::Query as _,
        test::client::{ExpectedUrl, SingleTestClient},
    };

    #[test]
    fn test_check_user_saved_albums_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("me/albums/contains")
            .add_query_params(&[(
                "ids",
                "382ObEPsp2rxGrnsizN5TX,1A2GTWGtFfWp7KSQTwWOyo,2noRn2Aes5aoNVsU6iWThc",
            )])
            .build();

        let expected_response = [false, false, false];

        let client = SingleTestClient::new_json(endpoint, &expected_response);

        let endpoint = CheckUserSavedAlbums::from([
            "382ObEPsp2rxGrnsizN5TX",
            "1A2GTWGtFfWp7KSQTwWOyo",
            "2noRn2Aes5aoNVsU6iWThc",
        ]);

        let result: Vec<bool> = endpoint.query(&client).unwrap();

        assert_eq!(result, expected_response);
    }
}
