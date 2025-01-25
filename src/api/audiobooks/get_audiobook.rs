use crate::api::prelude::*;

/// Get Spotify catalog information for a single audiobook.
/// Audiobooks are only available within the US, UK, Canada, Ireland, New Zealand and Australia markets.
#[derive(Debug, Builder, Clone, Endpoint)]
#[endpoint(method = GET, path = "audiobooks/{id}")]
pub struct GetAudiobook {
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the audiobook.
    #[builder(setter(into))]
    pub id: String,

    /// An [ISO 3166-1 alpha-2 country code](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2).
    /// If a country code is specified, only content that is available in that market will be returned.
    /// If a valid user access token is specified in the request header, the country associated with the user account will take priority over this parameter.
    ///
    /// # Notes
    /// If neither market or user country are provided, the content is considered unavailable for the client.
    /// Users can view the country that is associated with their account in the [account settings](https://www.spotify.com/account/overview/).
    #[builder(setter(into, strip_option), default)]
    pub market: Option<Market>,
}

impl GetAudiobook {
    pub fn builder() -> GetAudiobookBuilder {
        GetAudiobookBuilder::default()
    }
}

impl<T: Into<String>> From<T> for GetAudiobook {
    fn from(id: T) -> Self {
        Self {
            id: id.into(),
            market: None,
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
    fn test_get_audiobook_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("audiobooks/7iHfbu1YPACw6oZPAFJtqe")
            .build()
            .unwrap();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GetAudiobook::from("7iHfbu1YPACw6oZPAFJtqe");

        api::ignore(endpoint).query(&client).unwrap();
    }
}
