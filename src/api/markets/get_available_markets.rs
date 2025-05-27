use crate::api::prelude::*;

/// Get the list of markets where Spotify is available.
#[derive(Default, Debug, Clone)]
pub struct GetAvailableMarkets;

impl Endpoint for GetAvailableMarkets {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "markets".into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        api::{self, Query as _},
        model::Markets,
        test::client::{ExpectedUrl, SingleTestClient},
    };

    #[test]
    fn test_get_available_markets_endpoint() {
        let endpoint = ExpectedUrl::builder().endpoint("markets").build();
        let client = SingleTestClient::new_raw(endpoint, "");
        api::ignore(GetAvailableMarkets).query(&client).unwrap();
    }

    #[test]
    fn test_get_available_markets_endpoint_with_response() {
        let endpoint = ExpectedUrl::builder().endpoint("markets").build();
        let client = SingleTestClient::new_raw(endpoint, r#"{"markets": ["CA", "BR", "IT"]}"#);
        let response: Markets = GetAvailableMarkets.query(&client).unwrap();
        for genre in ["CA", "BR", "IT"] {
            assert!(response.markets.contains(&genre.to_owned()));
        }
    }
}
