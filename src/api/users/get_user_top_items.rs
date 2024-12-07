use crate::{
    api::prelude::*,
    model::{TimeRange, TopItemType},
};

/// Get the current user's top artists or tracks based on calculated affinity.
#[derive(Debug, Builder, Clone, Endpoint)]
#[endpoint(method = GET, path = "me/top/{type_}")]
pub struct GetUserTopItems {
    /// The type of entity to return. Valid values: artists or tracks
    type_: TopItemType,

    /// Over what time frame the affinities are computed.
    ///
    /// Valid values:
    ///
    /// `long_term` (calculated from ~1 year of data and including all new data as it becomes available),
    ///
    /// `medium_term` (approximately last 6 months),
    ///
    /// `short_term` (approximately last 4 weeks). Default: `medium_term`
    #[builder(setter(strip_option), default)]
    time_range: Option<TimeRange>,
}

impl GetUserTopItems {
    pub fn builder() -> GetUserTopItemsBuilder {
        GetUserTopItemsBuilder::default()
    }
}

impl Pageable for GetUserTopItems {}

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
            .build()
            .unwrap();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GetUserTopItems::builder()
            .type_(TopItemType::Tracks)
            .time_range(TimeRange::MediumTerm)
            .build()
            .unwrap();

        api::ignore(endpoint).query(&client).unwrap();
    }
}
