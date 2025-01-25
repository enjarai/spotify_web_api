use crate::api::prelude::*;

/// Get the object currently being played on the user's Spotify account.
#[derive(Debug, Default, Builder, Clone, Endpoint)]
#[endpoint(method = GET, path = "me/player/currently-playing")]
pub struct GetCurrentlyPlayingTrack {
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

impl GetCurrentlyPlayingTrack {
    pub fn builder() -> GetCurrentlyPlayingTrackBuilder {
        GetCurrentlyPlayingTrackBuilder::default()
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
    fn test_get_currently_playing_track_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("me/player/currently-playing")
            .build()
            .unwrap();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GetCurrentlyPlayingTrack::default();

        api::ignore(endpoint).query(&client).unwrap();
    }
}
