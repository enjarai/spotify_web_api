use crate::api::Endpoint;
use http::Method;
use std::borrow::Cow;

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
