use crate::api::prelude::*;

/// Seeks to the given position in the user’s currently playing track.
/// This API only works for users who have Spotify Premium.
/// The order of execution is not guaranteed when you use this API with other Player API endpoints.
#[derive(Default, Debug, Clone, Builder, Endpoint)]
#[endpoint(method = PUT, path = "me/player/seek")]
pub struct SeekToPosition {
    /// The id of the device this command is targeting. If not supplied, the user's currently active device is the target.
    #[builder(setter(into, strip_option), default)]
    pub device_id: Option<String>,

    /// The position in milliseconds to seek to.
    /// Passing in a position that is greater than the length of the track will cause the player to start playing the next song.
    #[builder(default)]
    pub position_ms: u32,
}

impl SeekToPosition {
    pub fn builder() -> SeekToPositionBuilder {
        SeekToPositionBuilder::default()
    }
}

impl From<u32> for SeekToPosition {
    fn from(position_ms: u32) -> Self {
        Self {
            position_ms,
            ..Self::default()
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
    fn test_seek_to_position_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("me/player/seek")
            .add_query_params(&[("position_ms", "1000")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");
        api::ignore(SeekToPosition::from(1000))
            .query(&client)
            .unwrap();
    }
}
