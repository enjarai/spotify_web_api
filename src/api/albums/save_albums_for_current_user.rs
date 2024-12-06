use crate::api::prelude::*;

/// Get Spotify catalog information for a single album.
#[derive(Debug, Builder, Clone)]
pub struct SaveAlbumsforCurrentUser {
    /// A list of the [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the albums.
    ids: Vec<String>,
}

#[allow(dead_code)]
impl SaveAlbumsforCurrentUserBuilder {
    fn id(&mut self, id: impl Into<String>) -> &mut Self {
        self.ids.get_or_insert_with(Vec::new).push(id.into());
        self
    }
}

impl SaveAlbumsforCurrentUser {
    pub fn builder() -> SaveAlbumsforCurrentUserBuilder {
        SaveAlbumsforCurrentUserBuilder::default()
    }
}

impl Endpoint for SaveAlbumsforCurrentUser {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "me/albums".into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        JsonParams::into_body(&json!({
            "ids": self.ids,
        }))
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
            .method(Method::PUT)
            .endpoint("me/albums")
            .content_type("application/json")
            .body_str(r#"{"ids":["7F50uh7oGitmAEScRKV6pD","27XW2QTeqZGOKlm2Dt0PvN"]}"#)
            .build()
            .unwrap();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = SaveAlbumsforCurrentUser::builder()
            .id("7F50uh7oGitmAEScRKV6pD")
            .id("27XW2QTeqZGOKlm2Dt0PvN")
            .build()
            .unwrap();

        api::ignore(endpoint).query(&client).unwrap();
    }
}
