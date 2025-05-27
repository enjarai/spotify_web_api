use crate::api::prelude::*;

/// Get Spotify catalog information for a single artist identified by their unique Spotify ID.
#[derive(Debug, Clone)]
pub struct GetArtist {
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) of the artist.
    pub id: String,
}

impl<T: Into<String>> From<T> for GetArtist {
    fn from(id: T) -> Self {
        Self { id: id.into() }
    }
}

impl Endpoint for GetArtist {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("artists/{}", self.id).into()
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
    fn test_get_artist_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("artists/5dRk8JyA2Tg9wL0iiTqbVu")
            .build();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint: GetArtist = GetArtist::from("5dRk8JyA2Tg9wL0iiTqbVu");

        api::ignore(endpoint).query(&client).unwrap();
    }
}
