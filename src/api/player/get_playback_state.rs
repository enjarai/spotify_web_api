use crate::{
    api::{prelude::*, Endpoint},
    model::AdditionalType,
};

/// Get information about the user’s current playback state, including track or episode, progress, and active device.
#[derive(Debug, Default, Builder, Clone)]
pub struct GetPlaybackState {
    /// An [ISO 3166-1 alpha-2 country code](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2).
    /// If a country code is specified, only content that is available in that market will be returned.
    /// If a valid user access token is specified in the request header, the country associated with the user account will take priority over this parameter.
    ///
    /// # Notes
    /// If neither market or user country are provided, the content is considered unavailable for the client.
    /// Users can view the country that is associated with their account in the [account settings](https://www.spotify.com/account/overview/).
    #[builder(setter(into, strip_option), default)]
    market: Option<Market>,

    // A list of item types that your client supports besides the default track type.
    #[builder(setter(strip_option), default)]
    additional_types: Option<Vec<AdditionalType>>,
}

impl GetPlaybackStateBuilder {
    pub fn additional_type(&mut self, additional_type: AdditionalType) -> &mut Self {
        match self.additional_types {
            Some(ref mut additional_types) => additional_types
                .get_or_insert_with(Vec::new)
                .push(additional_type),
            None => {
                self.additional_types = Some(Some(vec![additional_type]));
            }
        }

        self
    }
}

impl GetPlaybackState {
    pub fn builder() -> GetPlaybackStateBuilder {
        GetPlaybackStateBuilder::default()
    }
}

impl Endpoint for GetPlaybackState {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "me/player".into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();

        let types = self.additional_types.as_ref().map(|types| {
            types
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(",")
        });

        params.push_opt("market", self.market.as_ref());
        params.push_opt("additional_types", types);
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
    fn test_get_playback_state_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("me/player")
            .build()
            .unwrap();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GetPlaybackState::default();

        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn test_get_playback_state_endpoint_with_additional_types() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("me/player")
            .add_query_params(&[("additional_types", "track,episode")])
            .build()
            .unwrap();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GetPlaybackState::builder()
            .additional_type(AdditionalType::Track)
            .additional_type(AdditionalType::Episode)
            .build()
            .unwrap();

        api::ignore(endpoint).query(&client).unwrap();
    }
}
