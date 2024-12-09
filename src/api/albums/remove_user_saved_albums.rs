use crate::api::prelude::*;

/// Remove one or more albums from the current user's 'Your Music' library.
#[derive(Debug, Builder, Clone, Endpoint)]
#[endpoint(method = DELETE, path = "me/albums")]
pub struct RemoveUserSavedAlbums {
    /// A list of [Spotify IDs](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the albums.
    ids: Vec<String>,
}

impl RemoveUserSavedAlbumsBuilder {
    pub fn id(&mut self, id: impl Into<String>) -> &mut Self {
        self.ids.get_or_insert_with(Vec::new).push(id.into());
        self
    }
}

impl RemoveUserSavedAlbums {
    pub fn builder() -> RemoveUserSavedAlbumsBuilder {
        RemoveUserSavedAlbumsBuilder::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        api::{self, Query as _},
        test::client::{ExpectedUrl, SingleTestClient},
    };
    use http::Method;

    #[test]
    fn test_remove_user_saved_albums_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::DELETE)
            .endpoint("me/albums")
            .add_query_params(&[("ids", "7F50uh7oGitmAEScRKV6pD,27XW2QTeqZGOKlm2Dt0PvN")])
            .build()
            .unwrap();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = RemoveUserSavedAlbums::builder()
            .id("7F50uh7oGitmAEScRKV6pD")
            .id("27XW2QTeqZGOKlm2Dt0PvN")
            .build()
            .unwrap();

        api::ignore(endpoint).query(&client).unwrap();
    }
}
