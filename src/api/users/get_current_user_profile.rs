use crate::api::prelude::*;

/// Get detailed profile information about the current user (including the current user's username).
#[derive(Default, Debug, Clone, Endpoint)]
#[endpoint(method = GET, path = "me")]
pub struct GetCurrentUserProfile;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        api::{self, Query as _},
        test::client::{ExpectedUrl, SingleTestClient},
    };

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder().endpoint("me").build().unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");
        api::ignore(GetCurrentUserProfile).query(&client).unwrap();
    }
}
