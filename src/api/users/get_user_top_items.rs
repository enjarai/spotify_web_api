use crate::{
    api::prelude::*,
    model::{TimeRange, TopItemType},
};

/// Get the current user's top artists or tracks based on calculated affinity.
#[derive(Debug, Clone)]
pub struct GetUserTopItems {
    /// The type of entity to return. Valid values: artists or tracks
    pub type_: TopItemType,

    /// Over what time frame the affinities are computed.
    ///
    /// Valid values:
    ///
    /// `long_term` (calculated from ~1 year of data and including all new data as it becomes available),
    ///
    /// `medium_term` (approximately last 6 months),
    ///
    /// `short_term` (approximately last 4 weeks). Default: `medium_term`
    pub time_range: Option<TimeRange>,
}

impl Pageable for GetUserTopItems {}

impl From<TopItemType> for GetUserTopItems {
    fn from(type_: TopItemType) -> Self {
        Self {
            type_,
            time_range: None,
        }
    }
}

impl Endpoint for GetUserTopItems {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("me/top/{}", self.type_).into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();
        params.push_opt("time_range", self.time_range.as_ref());
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
    fn test_get_user_top_items_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("me/top/tracks")
            .add_query_params(&[("time_range", "medium_term")])
            .build();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GetUserTopItems {
            type_: TopItemType::Tracks,
            time_range: Some(TimeRange::MediumTerm),
        };

        api::ignore(endpoint).query(&client).unwrap();
    }
}
