use crate::api::prelude::*;

/// Transfer playback to a new device and optionally begin playback.
/// This API only works for users who have Spotify Premium.
/// The order of execution is not guaranteed when you use this API with other Player API endpoints.
#[derive(Debug, Clone)]
pub struct TransferPlayback {
    /// The ID of the device on which playback should be started/transferred.
    ///
    /// # Note
    /// Although an array is accepted, only a single `device_id` is currently supported. Supplying more than one will return 400 Bad Request
    pub device_ids: Vec<String>,

    /// `true`: ensure playback happens on new device.
    /// `false` or not provided: keep the current playback state.
    pub play: bool,
}

impl<T: Into<String>> From<T> for TransferPlayback {
    fn from(id: T) -> Self {
        Self {
            device_ids: vec![id.into()],
            play: false,
        }
    }
}

impl Endpoint for TransferPlayback {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "me/player".into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        JsonParams::into_body(&serde_json::json!({
            "device_ids": self.device_ids,
            "play": self.play,
        }))
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
    fn test_transfer_playback_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("me/player")
            .method(Method::PUT)
            .content_type("application/json")
            .body_str(r#"{"device_ids":["74ASZWbe4lXaubB36ztrGX"],"play":false}"#)
            .build();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = TransferPlayback::from("74ASZWbe4lXaubB36ztrGX");

        api::ignore(endpoint).query(&client).unwrap();
    }
}
