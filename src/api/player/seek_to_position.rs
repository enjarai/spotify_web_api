use crate::api::prelude::*;

/// Seeks to the given position in the user’s currently playing track.
/// This API only works for users who have Spotify Premium.
/// The order of execution is not guaranteed when you use this API with other Player API endpoints.
#[derive(Default, Debug, Clone)]
pub struct SeekToPosition {
    /// The id of the device this command is targeting. If not supplied, the user's currently active device is the target.
    pub device_id: Option<String>,

    /// The position in milliseconds to seek to.
    /// Passing in a position that is greater than the length of the track will cause the player to start playing the next song.
    pub position_ms: u32,
}

impl From<u32> for SeekToPosition {
    fn from(position_ms: u32) -> Self {
        Self {
            position_ms,
            ..Self::default()
        }
    }
}

impl Endpoint for SeekToPosition {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "me/player/seek".into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();
        params.push("position_ms", &self.position_ms);
        params.push_opt("device_id", self.device_id.as_ref());
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
    fn test_seek_to_position_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("me/player/seek")
            .add_query_params(&[("position_ms", "1000")])
            .build();
        let client = SingleTestClient::new_raw(endpoint, "");
        api::ignore(SeekToPosition::from(1000))
            .query(&client)
            .unwrap();
    }
}
