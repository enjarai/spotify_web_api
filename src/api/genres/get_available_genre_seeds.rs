use crate::api::prelude::*;

/// Retrieve a list of available genres seed parameter values for [recommendations](https://developer.spotify.com/documentation/web-api/reference/get-recommendations).
#[derive(Default, Debug, Clone, Endpoint)]
#[endpoint(method = GET, path = "recommendations/available-genre-seeds")]
pub struct GetAvailableGenreSeeds;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        api::{self, Query as _},
        model::Genres,
        test::client::{ExpectedUrl, SingleTestClient},
    };

    #[test]
    fn test_get_available_genre_seeds_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("recommendations/available-genre-seeds")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");
        api::ignore(GetAvailableGenreSeeds).query(&client).unwrap();
    }

    #[test]
    fn test_get_available_genre_seeds_endpoint_with_response() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("recommendations/available-genre-seeds")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, r#"{"genres": ["alternative", "samba"]}"#);
        let response: Genres = GetAvailableGenreSeeds.query(&client).unwrap();
        for genre in ["alternative", "samba"] {
            assert!(response.genres.contains(&genre.to_owned()));
        }
    }
}
