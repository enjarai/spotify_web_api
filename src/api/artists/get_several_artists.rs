use crate::api::prelude::*;

/// Get Spotify catalog information for several artists based on their Spotify IDs.
#[derive(Debug, Builder, Clone, Endpoint)]
#[endpoint(method = GET, path = "artists")]
pub struct GetSeveralArtists {
    /// A list of the [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the artists.
    ids: Vec<String>,
}

#[allow(dead_code)]
impl GetSeveralArtistsBuilder {
    fn id(&mut self, id: impl Into<String>) -> &mut Self {
        self.ids.get_or_insert_with(Vec::new).push(id.into());
        self
    }
}

impl GetSeveralArtists {
    pub fn builder() -> GetSeveralArtistsBuilder {
        GetSeveralArtistsBuilder::default()
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
    fn test_get_several_artists_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("artists")
            .add_query_params(&[(
                "ids",
                "2CIMQHirSU0MQqyYHq0eOx,57dN52uHvrHOxijzpIgu3E,1vCWHaC5f2uS3yhpwWbIA6",
            )])
            .build()
            .unwrap();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GetSeveralArtists::builder()
            .id("2CIMQHirSU0MQqyYHq0eOx")
            .id("57dN52uHvrHOxijzpIgu3E")
            .id("1vCWHaC5f2uS3yhpwWbIA6")
            .build()
            .unwrap();

        api::ignore(endpoint).query(&client).unwrap();
    }
}
