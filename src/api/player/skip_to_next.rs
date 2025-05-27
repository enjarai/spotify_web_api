use crate::api::prelude::*;

/// Skips to next track in the user’s queue.
/// This API only works for users who have Spotify Premium.
/// The order of execution is not guaranteed when you use this API with other Player API endpoints.
#[derive(Default, Debug, Clone, Endpoint)]
#[endpoint(method = POST, path = "me/player/next")]
pub struct SkipToNext {
    /// The id of the device this command is targeting. If not supplied, the user's currently active device is the target.
    pub device_id: Option<String>,
}

impl<T: Into<String>> From<T> for SkipToNext {
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
    fn test_skip_to_next_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("me/player/next")
            .build();
        let client = SingleTestClient::new_raw(endpoint, "");
        api::ignore(SkipToNext::default()).query(&client).unwrap();
    }
}
