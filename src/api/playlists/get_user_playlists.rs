use crate::api::prelude::*;

/// Get a list of the playlists owned or followed by a Spotify user.
#[derive(Debug, Clone)]
pub struct GetUserPlaylists {
    /// The user's [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids).
    pub id: String,
}

impl GetUserPlaylists {
    pub fn new(id: impl Into<String>) -> Self {
        Self::from(id)
    }
}

impl Pageable for GetUserPlaylists {}

impl<T: Into<String>> From<T> for GetUserPlaylists {
    fn from(id: T) -> Self {
        Self { id: id.into() }
    }
}

impl Endpoint for GetUserPlaylists {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("users/{}/playlists", self.id).into()
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
    fn test_get_user_playlists_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("users/smedjan/playlists")
            .build();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GetUserPlaylists::new("smedjan");

        api::ignore(endpoint).query(&client).unwrap();
    }
}
