use crate::api::prelude::*;

/// Get Spotify catalog information for a single episode identified by its unique Spotify ID.
#[derive(Debug, Clone, Endpoint)]
#[endpoint(method = GET, path = "episodes/{id}")]
pub struct GetEpisode {
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the episode.
    pub id: String,

    /// An [ISO 3166-1 alpha-2 country code](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2).
    /// If a country code is specified, only content that is available in that market will be returned.
    /// If a valid user access token is specified in the request header, the country associated with the user account will take priority over this parameter.
    ///
    /// # Notes
    /// If neither market or user country are provided, the content is considered unavailable for the client.
    /// Users can view the country that is associated with their account in the [account settings](https://www.spotify.com/account/overview/).
    pub market: Option<Market>,
}

impl<T: Into<String>> From<T> for GetEpisode {
    fn from(id: T) -> Self {
        Self {
            id: id.into(),
            market: None,
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

    #[test]
    fn test_get_episode_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("episodes/512ojhOuo1ktJprKbVcKyQ")
            .build();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GetEpisode::from("512ojhOuo1ktJprKbVcKyQ");

        api::ignore(endpoint).query(&client).unwrap();
    }
}
