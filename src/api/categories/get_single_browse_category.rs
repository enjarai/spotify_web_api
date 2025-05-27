use crate::api::prelude::*;

/// Get a single category used to tag items in Spotify (on, for example, the Spotify player’s “Browse” tab).
#[derive(Debug, Clone)]
pub struct GetSingleBrowseCategory {
    /// The [Spotify category ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the category.
    pub id: String,

    /// The desired language, consisting of an [ISO 639-1](http://en.wikipedia.org/wiki/ISO_639-1) language code and an [ISO 3166-1 alpha-2 country code](http://en.wikipedia.org/wiki/ISO_3166-1_alpha-2), joined by an underscore. For example: `es_MX`, meaning "Spanish (Mexico)". Provide this parameter if you want the category strings returned in a particular language.
    ///
    /// # Notes
    /// if locale is not supplied, or if the specified language is not available, the category strings returned will be in the Spotify default language (American English).
    ///
    /// Example: `sv_SE`
    pub locale: Option<String>,
}

impl<T: Into<String>> From<T> for GetSingleBrowseCategory {
    fn from(id: T) -> Self {
        Self {
            id: id.into(),
            locale: None,
        }
    }
}

impl Endpoint for GetSingleBrowseCategory {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("browse/categories/{}", self.id).into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();
        params.push_opt("locale", self.locale.as_ref());
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
    fn test_get_single_browse_category_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("browse/categories/dinner")
            .build();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GetSingleBrowseCategory::from("dinner");

        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn test_get_single_browse_category_endpoint_with_locale() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("browse/categories/dinner")
            .add_query_params(&[("locale", "sv_SE")])
            .build();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GetSingleBrowseCategory {
            id: "dinner".to_owned(),
            locale: Some("sv_SE".to_owned()),
        };

        api::ignore(endpoint).query(&client).unwrap();
    }
}
