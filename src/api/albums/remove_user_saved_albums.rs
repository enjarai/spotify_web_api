use crate::api::prelude::*;

/// Get Spotify catalog information for a single album.
#[derive(Debug, Builder, Clone, Endpoint)]
#[endpoint(method = DELETE, path = "me/albums")]
pub struct RemoveUserSavedAlbums {
    /// A list of the [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the albums.
    #[endpoint(body)]
    ids: Vec<String>,
}

#[allow(dead_code)]
impl RemoveUserSavedAlbumsBuilder {
    fn id(&mut self, id: impl Into<String>) -> &mut Self {
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
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::DELETE)
            .endpoint("me/albums")
            .content_type("application/json")
            .body_str(r#"{"ids":["7F50uh7oGitmAEScRKV6pD","27XW2QTeqZGOKlm2Dt0PvN"]}"#)
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
