use crate::api::prelude::*;

/// Get a list of categories used to tag items in Spotify (on, for example, the Spotify player’s “Browse” tab).
#[derive(Debug, Default, Clone, Endpoint)]
#[endpoint(method = GET, path = "browse/categories")]
pub struct GetSeveralBrowseCategories {
    /// The desired language, consisting of an [ISO 639-1](http://en.wikipedia.org/wiki/ISO_639-1) language code and an [ISO 3166-1 alpha-2 country code](http://en.wikipedia.org/wiki/ISO_3166-1_alpha-2), joined by an underscore. For example: `es_MX`, meaning "Spanish (Mexico)". Provide this parameter if you want the category strings returned in a particular language.
    ///
    /// Note: if locale is not supplied, or if the specified language is not available, the category strings returned will be in the Spotify default language (American English).
    ///
    /// Example: `sv_SE`
    pub locale: Option<String>,
}

impl GetSeveralBrowseCategories {
    pub fn with_locale(locale: impl Into<String>) -> Self {
        Self {
            locale: Some(locale.into()),
        }
    }
}

impl Pageable for GetSeveralBrowseCategories {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        api::{self, Query as _},
        test::client::{ExpectedUrl, SingleTestClient},
    };

    #[test]
    fn test_get_several_browse_categories_endpoint() {
        let endpoint = ExpectedUrl::builder().endpoint("browse/categories").build();

        let client = SingleTestClient::new_raw(endpoint, "");

        api::ignore(GetSeveralBrowseCategories::default())
            .query(&client)
            .unwrap();
    }

    #[test]
    fn test_get_several_browse_categories_endpoint_with_locale() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("browse/categories")
            .add_query_params(&[("locale", "sv_SE")])
            .build();

        let client = SingleTestClient::new_raw(endpoint, "");

        api::ignore(GetSeveralBrowseCategories::with_locale("sv_SE"))
            .query(&client)
            .unwrap();
    }
}
