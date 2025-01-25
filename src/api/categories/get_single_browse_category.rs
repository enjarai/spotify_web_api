use crate::api::prelude::*;

/// Get a single category used to tag items in Spotify (on, for example, the Spotify player’s “Browse” tab).
#[derive(Debug, Builder, Clone, Endpoint)]
#[endpoint(method = GET, path = "browse/categories/{id}")]
pub struct GetSingleBrowseCategory {
    /// The [Spotify category ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the category.
    #[builder(setter(into))]
    pub id: String,

    /// The desired language, consisting of an [ISO 639-1](http://en.wikipedia.org/wiki/ISO_639-1) language code and an [ISO 3166-1 alpha-2 country code](http://en.wikipedia.org/wiki/ISO_3166-1_alpha-2), joined by an underscore. For example: `es_MX`, meaning "Spanish (Mexico)". Provide this parameter if you want the category strings returned in a particular language.
    ///
    /// Note: if locale is not supplied, or if the specified language is not available, the category strings returned will be in the Spotify default language (American English).
    ///
    /// Example: `sv_SE`
    #[builder(setter(into, strip_option), default)]
    pub locale: Option<String>,
}

impl GetSingleBrowseCategory {
    pub fn builder() -> GetSingleBrowseCategoryBuilder {
        GetSingleBrowseCategoryBuilder::default()
    }
}

impl<T: Into<String>> From<T> for GetSingleBrowseCategory {
    fn from(id: T) -> Self {
        Self {
            id: id.into(),
            locale: None,
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
    fn test_get_single_browse_category_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("browse/categories/dinner")
            .build()
            .unwrap();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GetSingleBrowseCategory::from("dinner");

        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn test_get_single_browse_category_endpoint_with_locale() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("browse/categories/dinner")
            .add_query_params(&[("locale", "sv_SE")])
            .build()
            .unwrap();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GetSingleBrowseCategory::builder()
            .id("dinner")
            .locale("sv_SE")
            .build()
            .unwrap();

        api::ignore(endpoint).query(&client).unwrap();
    }
}
