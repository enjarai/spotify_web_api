use crate::api::prelude::*;

/// Get Spotify catalog information for several audiobooks identified by their Spotify IDs.
/// Audiobooks are only available within the US, UK, Canada, Ireland, New Zealand and Australia markets.
#[derive(Debug, Clone, Endpoint)]
#[endpoint(method = GET, path = "audiobooks")]
pub struct GetSeveralAudiobooks {
    /// A list of [Spotify IDs](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the audiobooks.
    pub ids: Vec<String>,

    /// An [ISO 3166-1 alpha-2 country code](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2).
    /// If a country code is specified, only content that is available in that market will be returned.
    /// If a valid user access token is specified in the request header, the country associated with the user account will take priority over this parameter.
    ///
    /// # Notes
    /// If neither market or user country are provided, the content is considered unavailable for the client.
    /// Users can view the country that is associated with their account in the [account settings](https://www.spotify.com/account/overview/).
    pub market: Option<Market>,
}

impl<T, I> From<I> for GetSeveralAudiobooks
where
    I: IntoIterator<Item = T>,
    T: Into<String>,
{
    fn from(ids: I) -> Self {
        Self {
            ids: ids.into_iter().map(Into::into).collect(),
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
    fn test_get_several_audiobooks_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("audiobooks")
            .add_query_params(&[(
                "ids",
                "18yVqkdbdRvS24c0Ilj2ci,1HGw3J3NxZO1TP1BTtVhpZ,7iHfbu1YPACw6oZPAFJtqe",
            )])
            .build();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GetSeveralAudiobooks::from([
            "18yVqkdbdRvS24c0Ilj2ci",
            "1HGw3J3NxZO1TP1BTtVhpZ",
            "7iHfbu1YPACw6oZPAFJtqe",
        ]);

        api::ignore(endpoint).query(&client).unwrap();
    }
}
