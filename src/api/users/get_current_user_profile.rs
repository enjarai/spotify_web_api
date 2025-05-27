use crate::api::prelude::*;

/// Get detailed profile information about the current user (including the current user's username).
#[derive(Default, Debug, Clone)]
pub struct GetCurrentUserProfile;

impl Endpoint for GetCurrentUserProfile {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "me".into()
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
    fn test_get_current_user_profile_endpoint() {
        let endpoint = ExpectedUrl::builder().endpoint("me").build();
        let client = SingleTestClient::new_raw(endpoint, "");
        api::ignore(GetCurrentUserProfile).query(&client).unwrap();
    }
}
