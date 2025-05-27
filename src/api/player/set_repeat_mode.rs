use crate::{api::prelude::*, model::RepeatState};

/// Set the repeat mode for the user's playback.
/// This API only works for users who have Spotify Premium.
/// The order of execution is not guaranteed when you use this API with other Player API endpoints.
#[derive(Debug, Clone, Endpoint)]
#[endpoint(method = PUT, path = "me/player/repeat")]
pub struct SetRepeatMode {
    /// The id of the device this command is targeting. If not supplied, the user's currently active device is the target.
    pub device_id: Option<String>,

    pub state: RepeatState,
}

impl From<RepeatState> for SetRepeatMode {
    fn from(state: RepeatState) -> Self {
        Self {
            device_id: None,
            state,
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
    fn test_set_repeat_mode_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("me/player/repeat")
            .add_query_params(&[("state", "track")])
            .build();
        let client = SingleTestClient::new_raw(endpoint, "");
        api::ignore(SetRepeatMode::from(RepeatState::Track))
            .query(&client)
            .unwrap();
    }
}
