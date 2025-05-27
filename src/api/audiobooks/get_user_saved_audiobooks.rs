use crate::api::prelude::*;

/// Get a list of the audiobooks saved in the current Spotify user's 'Your Music' library.
#[derive(Default, Debug, Clone, Endpoint)]
#[endpoint(method = GET, path = "me/audiobooks")]
pub struct GetUserSavedAudiobooks;

impl Pageable for GetUserSavedAudiobooks {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        api::{self, Query as _},
        test::client::{ExpectedUrl, SingleTestClient},
    };

    #[test]
    fn test_get_user_saved_audiobooks_endpoint() {
        let endpoint = ExpectedUrl::builder().endpoint("me/audiobooks").build();
        let client = SingleTestClient::new_raw(endpoint, "");
        api::ignore(GetUserSavedAudiobooks).query(&client).unwrap();
    }
}
