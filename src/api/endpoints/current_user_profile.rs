use crate::api::Endpoint;
use http::Method;
use std::borrow::Cow;

#[derive(Default, Debug, Clone)]
pub struct CurrentUserProfileEndpoint;

impl Endpoint for CurrentUserProfileEndpoint {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "me".into()
    }
}
