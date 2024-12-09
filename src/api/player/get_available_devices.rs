use crate::api::prelude::*;

/// Get information about a user’s available Spotify Connect devices.
/// Some device models are not supported and will not be listed in the API response.
#[derive(Default, Debug, Clone, Endpoint)]
#[endpoint(method = GET, path = "me/player/devices")]
pub struct GetAvailableDevices;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        api::{self, Query as _},
        test::client::{ExpectedUrl, SingleTestClient},
    };

    #[test]
    fn test_get_available_devices_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("me/player/devices")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");
        api::ignore(GetAvailableDevices).query(&client).unwrap();
    }
}
