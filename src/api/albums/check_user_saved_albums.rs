use crate::api::prelude::*;

/// Check if one or more albums is already saved in the current Spotify user's 'Your Music' library.
#[derive(Debug, Builder, Clone)]
pub struct CheckUserSavedAlbums {
    /// A list of the [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the albums.
    ids: Vec<String>,
}

#[allow(dead_code)]
impl CheckUserSavedAlbumsBuilder {
    fn id(&mut self, id: impl Into<String>) -> &mut Self {
        self.ids.get_or_insert_with(Vec::new).push(id.into());
        self
    }
}

impl CheckUserSavedAlbums {
    pub fn builder() -> CheckUserSavedAlbumsBuilder {
        CheckUserSavedAlbumsBuilder::default()
    }
}

impl Endpoint for CheckUserSavedAlbums {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "me/albums/contains".into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();
        params.push("ids", &self.ids.join(","));
        params
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        api::Query as _,
        test::client::{ExpectedUrl, SingleTestClient},
    };

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("me/albums/contains")
            .add_query_params(&[(
                "ids",
                "382ObEPsp2rxGrnsizN5TX,1A2GTWGtFfWp7KSQTwWOyo,2noRn2Aes5aoNVsU6iWThc",
            )])
            .build()
            .unwrap();

        let client = SingleTestClient::new_json(endpoint, &[false, false, false]);

        let endpoint = CheckUserSavedAlbums::builder()
            .id("382ObEPsp2rxGrnsizN5TX")
            .id("1A2GTWGtFfWp7KSQTwWOyo")
            .id("2noRn2Aes5aoNVsU6iWThc")
            .build()
            .unwrap();

        let result: Vec<bool> = endpoint.query(&client).unwrap();

        assert!(result == [false, false, false]);
    }
}
