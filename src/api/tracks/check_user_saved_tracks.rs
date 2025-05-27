use crate::api::prelude::*;

/// Check if one or more tracks are already saved in the current Spotify user's library.
#[derive(Debug, Clone)]
pub struct CheckUserSavedTracks {
    /// A list of [Spotify IDs](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the tracks.
    pub ids: Vec<String>,
}

impl<T, I> From<I> for CheckUserSavedTracks
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

impl Endpoint for CheckUserSavedTracks {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "me/tracks/contains".into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();
        params.push("ids", &self.ids.join(","));
        params
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
            .build();

        let expected_response = [false, false];

        let client = SingleTestClient::new_json(endpoint, &expected_response);

        let endpoint =
            CheckUserSavedTracks::from(["39joRyXYyjSpI6nKZHyWmH", "5mPY98zmeNSp8cmrRtdUW3"]);

        let result: Vec<bool> = endpoint.query(&client).unwrap();

        assert_eq!(result, expected_response);
    }
}
