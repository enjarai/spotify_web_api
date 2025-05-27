use crate::api::prelude::*;

/// Get public profile information about a Spotify user.
#[derive(Default, Debug, Clone, Endpoint)]
#[endpoint(method = GET, path = "users/{id}")]
pub struct GetUserProfile {
    /// The user's [Spotify user ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids).
    pub id: String,
}

impl GetUserProfile {
    pub fn new(id: impl Into<String>) -> Self {
        Self::from(id)
    }
}

impl<T: Into<String>> From<T> for GetUserProfile {
    fn from(id: T) -> Self {
        Self { id: id.into() }
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
    fn test_get_user_profile_endpoint() {
        let endpoint = ExpectedUrl::builder().endpoint("users/severino246").build();
        let client = SingleTestClient::new_raw(endpoint, "");
        api::ignore(GetUserProfile::from("severino246"))
            .query(&client)
            .unwrap();
    }
}
