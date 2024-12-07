use crate::api::prelude::*;

/// Get Spotify catalog information for a single artist identified by their unique Spotify ID.
#[derive(Debug, Builder, Clone, Endpoint)]
#[endpoint(method = GET, path = "artists/{id}")]
pub struct GetArtist {
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) of the artist.
    #[builder(setter(into))]
    id: String,
}

impl GetArtist {
    pub fn builder() -> GetArtistBuilder {
        GetArtistBuilder::default()
    }
}

impl From<&str> for GetArtist {
    fn from(id: &str) -> Self {
        Self { id: id.to_owned() }
    }
}

impl From<String> for GetArtist {
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
    fn test_get_artist_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("artists/5dRk8JyA2Tg9wL0iiTqbVu")
            .build()
            .unwrap();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GetArtist::from("5dRk8JyA2Tg9wL0iiTqbVu");

        api::ignore(endpoint).query(&client).unwrap();
    }
}
