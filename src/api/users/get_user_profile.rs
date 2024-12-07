use crate::api::prelude::*;

/// Get public profile information about a Spotify user.
#[derive(Default, Debug, Clone, Endpoint)]
#[endpoint(method = GET, path = "users/{id}")]
pub struct GetUserProfile {
    /// The user's [Spotify user ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids).
    id: String,
}

impl From<&str> for GetUserProfile {
    fn from(id: &str) -> Self {
        Self { id: id.to_owned() }
    }
}

impl From<String> for GetUserProfile {
    fn from(id: String) -> Self {
        Self { id }
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
        let endpoint = ExpectedUrl::builder()
            .endpoint("users/severino246")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");
        api::ignore(GetUserProfile::from("severino246"))
            .query(&client)
            .unwrap();
    }
}
