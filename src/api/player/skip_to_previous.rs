use crate::api::prelude::*;

/// Skips to previous track in the user’s queue.
/// This API only works for users who have Spotify Premium.
/// The order of execution is not guaranteed when you use this API with other Player API endpoints.
#[derive(Default, Debug, Clone)]
pub struct SkipToPrevious {
    /// The id of the device this command is targeting. If not supplied, the user's currently active device is the target.
    pub device_id: Option<String>,
}

impl<T: Into<String>> From<T> for SkipToPrevious {
    fn from(device_id: T) -> Self {
        Self {
            device_id: Some(device_id.into()),
        }
    }
}

impl Endpoint for SkipToPrevious {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "me/player/previous".into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();
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
    fn test_skip_to_previous_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("me/player/previous")
            .build();
        let client = SingleTestClient::new_raw(endpoint, "");
        api::ignore(SkipToPrevious::default())
            .query(&client)
            .unwrap();
    }
}
