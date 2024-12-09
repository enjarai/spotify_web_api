use crate::{
    api::{common::path_escaped, prelude::*, Endpoint},
    model::{IncludeExternalType, SearchType},
};

/// Get Spotify catalog information about albums, artists, playlists, tracks, shows, episodes or audiobooks that match a keyword string.
/// Audiobooks are only available within the US, UK, Canada, Ireland, New Zealand and Australia markets.
#[derive(Debug, Builder, Clone)]
pub struct SearchForItem {
    /// Your search query.
    #[builder(setter(into))]
    q: String,

    /// A list of item types to search across. Search results include hits from all the specified item types.
    type_: Vec<SearchType>,

    /// An [ISO 3166-1 alpha-2 country code](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2).
    /// If a country code is specified, only content that is available in that market will be returned.
    /// If a valid user access token is specified in the request header, the country associated with the user account will take priority over this parameter.
    ///
    /// # Notes
    /// If neither market or user country are provided, the content is considered unavailable for the client.
    /// Users can view the country that is associated with their account in the [account settings](https://www.spotify.com/account/overview/).
    #[builder(setter(into, strip_option), default)]
    market: Option<Market>,

    /// If `include_external=audio` is specified it signals that the client can play externally hosted audio content, and marks the content as playable in the response.
    /// By default externally hosted audio content is marked as unplayable in the response.
    #[builder(setter(strip_option), default)]
    include_external: Option<IncludeExternalType>,
}

impl SearchForItemBuilder {
    pub fn add_type(&mut self, ty: SearchType) -> &mut Self {
        self.type_.get_or_insert_with(Vec::new).push(ty);
        self
    }
}

impl SearchForItem {
    pub fn builder() -> SearchForItemBuilder {
        SearchForItemBuilder::default()
    }
}

impl Pageable for SearchForItem {}

impl Endpoint for SearchForItem {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "search".into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();
        params.push("q", &format!("{}", path_escaped(&self.q)));

        let type_str = self
            .type_
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",");

        params.push("type", &type_str);
        params.push_opt("market", self.market.as_ref());
        params.push_opt("include_external", self.include_external.as_ref());

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
    fn test_search_for_item_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("search")
            .add_query_params(&[("q", "remaster%20track:Doxy%20artist:Miles%20Davis")])
            .add_query_params(&[("type", "album")])
            .build()
            .unwrap();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = SearchForItem::builder()
            .q("remaster track:Doxy artist:Miles Davis")
            .add_type(SearchType::Album)
            .build()
            .unwrap();

        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn test_search_for_item_endpoint_with_include_external() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("search")
            .add_query_params(&[("q", "remaster%20track:Doxy%20artist:Miles%20Davis")])
            .add_query_params(&[("type", "album")])
            .add_query_params(&[("include_external", "audio")])
            .build()
            .unwrap();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = SearchForItem::builder()
            .q("remaster track:Doxy artist:Miles Davis")
            .add_type(SearchType::Album)
            .include_external(IncludeExternalType::Audio)
            .build()
            .unwrap();

        api::ignore(endpoint).query(&client).unwrap();
    }
}
