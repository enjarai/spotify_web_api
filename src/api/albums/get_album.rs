use crate::api::prelude::*;

/// Get Spotify catalog information for a single album.
#[derive(Debug, Builder, Clone)]
pub struct GetAlbum {
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) of the album.
    #[builder(setter(into))]
    id: String,
}

impl GetAlbum {
    pub fn builder() -> GetAlbumBuilder {
        GetAlbumBuilder::default()
    }
}

impl Endpoint for GetAlbum {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("albums/{}", self.id).into()
    }
}

impl From<&str> for GetAlbum {
    fn from(id: &str) -> Self {
        Self { id: id.to_owned() }
    }
}

impl From<String> for GetAlbum {
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
            .endpoint("albums/0oKvU088cLhKbbVvQc9lQF")
            .build()
            .unwrap();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GetAlbum::from("0oKvU088cLhKbbVvQc9lQF");

        api::ignore(endpoint).query(&client).unwrap();
    }
}
