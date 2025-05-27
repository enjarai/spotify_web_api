use crate::api::prelude::*;

/// Get a list of shows saved in the current Spotify user's library.
#[derive(Default, Debug, Clone)]
pub struct GetUserSavedShows;

impl Pageable for GetUserSavedShows {}

impl Endpoint for GetUserSavedShows {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "me/shows".into()
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
    fn test_get_user_saved_shows_endpoint() {
        let endpoint = ExpectedUrl::builder().endpoint("me/shows").build();
        let client = SingleTestClient::new_raw(endpoint, "");
        api::ignore(GetUserSavedShows).query(&client).unwrap();
    }
}
