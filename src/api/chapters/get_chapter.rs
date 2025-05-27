use crate::api::prelude::*;

/// Get Spotify catalog information for a single audiobook chapter.
/// Chapters are only available within the US, UK, Canada, Ireland, New Zealand and Australia markets.
#[derive(Debug, Clone)]
pub struct GetChapter {
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) of the chapter.
    pub id: String,

    /// An [ISO 3166-1 alpha-2 country code](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2).
    /// If a country code is specified, only content that is available in that market will be returned.
    /// If a valid user access token is specified in the request header, the country associated with the user account will take priority over this parameter.
    ///
    /// # Notes
    /// If neither market or user country are provided, the content is considered unavailable for the client.
    /// Users can view the country that is associated with their account in the [account settings](https://www.spotify.com/account/overview/).
    pub market: Option<Market>,
}

impl<T: Into<String>> From<T> for GetChapter {
    fn from(id: T) -> Self {
        Self {
            id: id.into(),
            market: None,
        }
    }
}

impl Endpoint for GetChapter {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("chapters/{}", self.id).into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();
        params.push_opt("market", self.market.as_ref());
        params
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
    fn test_get_chapter_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("chapters/0D5wENdkdwbqlrHoaJ9g29")
            .build();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GetChapter::from("0D5wENdkdwbqlrHoaJ9g29");

        api::ignore(endpoint).query(&client).unwrap();
    }
}
