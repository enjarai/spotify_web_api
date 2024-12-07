use crate::api::prelude::*;

/// Get a list of the albums saved in the current Spotify user's 'Your Music' library.
#[derive(Debug, Default, Clone, Endpoint)]
#[endpoint(method = GET, path = "me/albums")]
pub struct GetUserSavedAlbums {
    /// An [ISO 3166-1 alpha-2 country code](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2).
    /// If a country code is specified, only content that is available in that market will be returned.
    /// If a valid user access token is specified in the request header, the country associated with the user account will take priority over this parameter.
    ///
    /// # Notes
    /// If neither market or user country are provided, the content is considered unavailable for the client.
    /// Users can view the country that is associated with their account in the [account settings](https://www.spotify.com/account/overview/).
    market: Option<Market>,
}

impl GetUserSavedAlbums {
    pub fn with_market(market: Market) -> Self {
        Self {
            market: Some(market),
        }
    }
}

impl Pageable for GetUserSavedAlbums {}

impl From<&Market> for GetUserSavedAlbums {
    fn from(market: &Market) -> Self {
        Self {
            market: Some(market.to_owned()),
        }
    }
}

impl From<Market> for GetUserSavedAlbums {
    fn from(market: Market) -> Self {
        Self {
            market: Some(market),
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
    fn test_get_user_saved_albums_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("me/albums")
            .build()
            .unwrap();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GetUserSavedAlbums::default();

        api::ignore(endpoint).query(&client).unwrap();
    }
}
