use crate::api::prelude::*;

/// Get information about the user’s current playback state, including track or episode, progress, and active device.
#[derive(Debug, Default, Clone)]
pub struct GetPlaybackState {
    /// An [ISO 3166-1 alpha-2 country code](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2).
    /// If a country code is specified, only content that is available in that market will be returned.
    /// If a valid user access token is specified in the request header, the country associated with the user account will take priority over this parameter.
    ///
    /// # Notes
    /// If neither market or user country are provided, the content is considered unavailable for the client.
    /// Users can view the country that is associated with their account in the [account settings](https://www.spotify.com/account/overview/).
    pub market: Option<Market>,
}

impl Endpoint for GetPlaybackState {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "me/player".into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();
        params.push_opt("market", self.market.as_ref());
        params
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
    fn test_get_playback_state_endpoint() {
        let endpoint = ExpectedUrl::builder().endpoint("me/player").build();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GetPlaybackState::default();

        api::ignore(endpoint).query(&client).unwrap();
    }
}
