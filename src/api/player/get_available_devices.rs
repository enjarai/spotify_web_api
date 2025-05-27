use crate::api::prelude::*;

/// Get information about a user’s available Spotify Connect devices.
/// Some device models are not supported and will not be listed in the API response.
#[derive(Default, Debug, Clone)]
pub struct GetAvailableDevices;

impl Endpoint for GetAvailableDevices {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "me/player/devices".into()
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
    fn test_get_available_devices_endpoint() {
        let endpoint = ExpectedUrl::builder().endpoint("me/player/devices").build();
        let client = SingleTestClient::new_raw(endpoint, "");
        api::ignore(GetAvailableDevices).query(&client).unwrap();
    }
}
