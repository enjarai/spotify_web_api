use crate::api::prelude::*;

/// Toggle shuffle on or off for user’s playback.
/// This API only works for users who have Spotify Premium.
/// The order of execution is not guaranteed when you use this API with other Player API endpoints.
#[derive(Debug, Clone)]
pub struct TogglePlaybackShuffle {
    /// The id of the device this command is targeting. If not supplied, the user's currently active device is the target.
    pub device_id: Option<String>,

    // The shuffle state.
    pub state: bool,
}

impl From<bool> for TogglePlaybackShuffle {
    fn from(state: bool) -> Self {
        Self {
            device_id: None,
            state,
        }
    }
}

impl Endpoint for TogglePlaybackShuffle {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "me/player/shuffle".into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();
        params.push_opt("device_id", self.device_id.as_ref());
        params.push("state", &self.state);
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
    fn test_toggle_playback_shuffle_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("me/player/shuffle")
            .add_query_params(&[("state", "true")])
            .build();
        let client = SingleTestClient::new_raw(endpoint, "");
        api::ignore(TogglePlaybackShuffle::from(true))
            .query(&client)
            .unwrap();
    }
}
