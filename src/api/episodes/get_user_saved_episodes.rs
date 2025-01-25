use crate::api::prelude::*;

/// Get a list of the episodes saved in the current Spotify user's library.
///
/// This API endpoint is in beta and could change without warning. Please share any feedback that you have, or issues that you discover, in the [Spotify developer community forum](https://community.spotify.com/t5/Spotify-for-Developers/bd-p/Spotify_Developer).
#[derive(Default, Builder, Debug, Clone, Endpoint)]
#[endpoint(method = GET, path = "me/episodes")]
pub struct GetUserSavedEpisodes {
    /// An [ISO 3166-1 alpha-2 country code](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2).
    /// If a country code is specified, only content that is available in that market will be returned.
    /// If a valid user access token is specified in the request header, the country associated with the user account will take priority over this parameter.
    ///
    /// # Notes
    /// If neither market or user country are provided, the content is considered unavailable for the client.
    /// Users can view the country that is associated with their account in the [account settings](https://www.spotify.com/account/overview/).
    #[builder(setter(into, strip_option), default)]
    pub market: Option<Market>,
}

impl GetUserSavedEpisodes {
    pub fn builder() -> GetUserSavedEpisodesBuilder {
        GetUserSavedEpisodesBuilder::default()
    }
}

impl Pageable for GetUserSavedEpisodes {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        api::{self, Query as _},
        test::client::{ExpectedUrl, SingleTestClient},
    };

    #[test]
    fn test_get_user_saved_episode_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("me/episodes")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");
        let endpoint = GetUserSavedEpisodes::default();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
