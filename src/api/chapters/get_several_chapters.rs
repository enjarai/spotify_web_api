use crate::api::prelude::*;

/// Get Spotify catalog information for several audiobook chapters identified by their Spotify IDs.
/// Chapters are only available within the US, UK, Canada, Ireland, New Zealand and Australia markets.
#[derive(Debug, Builder, Clone, Endpoint)]
#[endpoint(method = GET, path = "chapters")]
pub struct GetSeveralChapters {
    /// A list of [Spotify IDs](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the chapters.
    pub ids: Vec<String>,

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

impl GetSeveralChaptersBuilder {
    pub fn id(&mut self, id: impl Into<String>) -> &mut Self {
        self.ids.get_or_insert_with(Vec::new).push(id.into());
        self
    }
}

impl GetSeveralChapters {
    pub fn builder() -> GetSeveralChaptersBuilder {
        GetSeveralChaptersBuilder::default()
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
    fn test_get_several_chapters_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("chapters")
            .add_query_params(&[(
                "ids",
                "0IsXVP0JmcB2adSE338GkK,3ZXb8FKZGU0EHALYX6uCzU,0D5wENdkdwbqlrHoaJ9g29",
            )])
            .build()
            .unwrap();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GetSeveralChapters::builder()
            .id("0IsXVP0JmcB2adSE338GkK")
            .id("3ZXb8FKZGU0EHALYX6uCzU")
            .id("0D5wENdkdwbqlrHoaJ9g29")
            .build()
            .unwrap();

        api::ignore(endpoint).query(&client).unwrap();
    }
}
