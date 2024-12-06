use crate::api::prelude::*;

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
    fn endpoint() {
        let endpoint = ExpectedUrl::builder().endpoint("me").build().unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");
        api::ignore(GetCurrentUserProfile).query(&client).unwrap();
    }
}
