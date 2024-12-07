use crate::api::prelude::*;

/// Get the list of markets where Spotify is available.
#[derive(Default, Debug, Clone, Endpoint)]
#[endpoint(method = GET, path = "markets")]
pub struct GetAvailableMarkets;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        api::{self, Query as _},
        model::Markets,
        test::client::{ExpectedUrl, SingleTestClient},
    };

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder().endpoint("markets").build().unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");
        api::ignore(GetAvailableMarkets).query(&client).unwrap();
    }

    #[test]
    fn response() {
        let endpoint = ExpectedUrl::builder().endpoint("markets").build().unwrap();
        let client = SingleTestClient::new_raw(endpoint, r#"{"markets": ["CA", "BR", "IT"]}"#);
        let response: Markets = GetAvailableMarkets.query(&client).unwrap();
        for genre in ["CA", "BR", "IT"] {
            assert!(response.markets.contains(&genre.to_owned()));
        }
    }
}
