use crate::api::prelude::*;

/// Get Spotify catalog information for a single track identified by its unique Spotify ID.
#[derive(Debug, Builder, Clone)]
pub struct GetTrack {
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) of the track.
    #[builder(setter(into))]
    id: String,

    /// An [ISO 3166-1 alpha-2 country code](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2).
    /// If a country code is specified, only content that is available in that market will be returned.
    /// If a valid user access token is specified in the request header, the country associated with the user account will take priority over this parameter.
    ///
    /// # Notes
    /// If neither market or user country are provided, the content is considered unavailable for the client.
    /// Users can view the country that is associated with their account in the [account settings](https://www.spotify.com/account/overview/).
    #[builder(setter(into, strip_option), default)]
    market: Option<String>,
}

impl GetTrack {
    pub fn builder() -> GetTrackBuilder {
        GetTrackBuilder::default()
    }
}

impl Endpoint for GetTrack {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("tracks/{}", self.id).into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();
        params.push_opt("market", self.market.as_deref());
        params
    }
}

impl From<&str> for GetTrack {
    fn from(id: &str) -> Self {
        Self {
            id: id.to_owned(),
            market: None,
        }
    }
}

impl From<String> for GetTrack {
    fn from(id: String) -> Self {
        Self { id, market: None }
    }
}
