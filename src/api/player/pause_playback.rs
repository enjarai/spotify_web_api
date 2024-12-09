use crate::api::prelude::*;

/// Pause playback on the user's account.
/// This API only works for users who have Spotify Premium.
/// The order of execution is not guaranteed when you use this API with other Player API endpoints.
#[derive(Default, Debug, Clone, Builder, Endpoint)]
#[endpoint(method = PUT, path = "me/player/pause")]
pub struct PausePlayback {
    /// The id of the device this command is targeting. If not supplied, the user's currently active device is the target.
    #[builder(setter(into, strip_option), default)]
    pub device_id: Option<String>,
}

impl PausePlayback {
    pub fn builder() -> PausePlaybackBuilder {
        PausePlaybackBuilder::default()
    }
}

impl<T: Into<String>> From<T> for PausePlayback {
    fn from(device_id: T) -> Self {
        Self {
            device_id: Some(device_id.into()),
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
    fn test_pause_playback_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("me/player/pause")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");
        api::ignore(PausePlayback::default())
            .query(&client)
            .unwrap();
    }
}
