use crate::api::prelude::*;

/// Save one or more shows to the current Spotify user's library.
#[derive(Debug, Builder, Clone, Endpoint)]
#[endpoint(method = PUT, path = "me/shows")]
pub struct SaveShowsforCurrentUser {
    /// A list of [Spotify IDs](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the shows.
    pub ids: Vec<String>,
}

impl SaveShowsforCurrentUserBuilder {
    pub fn id(&mut self, id: impl Into<String>) -> &mut Self {
        self.ids.get_or_insert_with(Vec::new).push(id.into());
        self
    }
}

impl SaveShowsforCurrentUser {
    pub fn builder() -> SaveShowsforCurrentUserBuilder {
        SaveShowsforCurrentUserBuilder::default()
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
    fn test_save_shows_for_current_user_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("me/shows")
            .add_query_params(&[("ids", "5CfCWKI5pZ28U0uOzXkDHe,5as3aKmN2k11yfDDDSrvaZ")])
            .build()
            .unwrap();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = SaveShowsforCurrentUser::builder()
            .id("5CfCWKI5pZ28U0uOzXkDHe")
            .id("5as3aKmN2k11yfDDDSrvaZ")
            .build()
            .unwrap();

        api::ignore(endpoint).query(&client).unwrap();
    }
}
