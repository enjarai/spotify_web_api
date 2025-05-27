use crate::api::prelude::*;

/// Get Spotify catalog information for several artists based on their Spotify IDs.
#[derive(Debug, Clone, Endpoint)]
#[endpoint(method = GET, path = "artists")]
pub struct GetSeveralArtists {
    /// A list of [Spotify IDs](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the artists.
    pub ids: Vec<String>,
}

impl<T, I> From<I> for GetSeveralArtists
where
    I: IntoIterator<Item = T>,
    T: Into<String>,
{
    fn from(ids: I) -> Self {
        Self {
            ids: ids.into_iter().map(Into::into).collect(),
        }
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
            .build();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GetSeveralArtists::from([
            "2CIMQHirSU0MQqyYHq0eOx",
            "57dN52uHvrHOxijzpIgu3E",
            "1vCWHaC5f2uS3yhpwWbIA6",
        ]);

        api::ignore(endpoint).query(&client).unwrap();
    }
}
