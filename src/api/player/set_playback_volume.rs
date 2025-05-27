use crate::api::{Endpoint, prelude::*};

/// Set the volume for the user’s current playback device.
/// This API only works for users who have Spotify Premium.
/// The order of execution is not guaranteed when you use this API with other Player API endpoints.
#[derive(Debug, Clone)]
pub struct SetPlaybackVolume {
    /// The id of the device this command is targeting. If not supplied, the user's currently active device is the target.
    pub device_id: Option<String>,

    /// The volume to set. Must be a value from 0 to 100 inclusive.
    pub volume_percent: u8,
}

impl Endpoint for SetPlaybackVolume {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "me/player/volume".into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();
        params.push_opt("device_id", self.device_id.as_ref());
        params.push(
            "volume_percent",
            &self.volume_percent.clamp(0, 100).to_string(),
        );
        params
    }
}

impl From<u8> for SetPlaybackVolume {
    fn from(volume_percent: u8) -> Self {
        Self {
            device_id: None,
            volume_percent,
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
    fn test_set_playback_volume_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("me/player/volume")
            .add_query_params(&[("volume_percent", "100")])
            .build();
        let client = SingleTestClient::new_raw(endpoint, "");
        api::ignore(SetPlaybackVolume::from(255))
            .query(&client)
            .unwrap();
    }
}
