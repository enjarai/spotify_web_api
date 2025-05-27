use crate::api::prelude::*;

/// Get the list of objects that make up the user's queue.
#[derive(Default, Debug, Clone)]
pub struct GetUserQueue;

impl Endpoint for GetUserQueue {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "me/player/queue".into()
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
    fn test_get_user_queue_endpoint() {
        let endpoint = ExpectedUrl::builder().endpoint("me/player/queue").build();
        let client = SingleTestClient::new_raw(endpoint, "");
        api::ignore(GetUserQueue).query(&client).unwrap();
    }
}
