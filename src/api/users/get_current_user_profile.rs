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
