use crate::{
    api::{Endpoint, prelude::*},
    model::QueryRange,
};

/// Get tracks from the current user's recently played tracks.
/// # Note:
/// Currently doesn't support podcast episodes.
#[derive(Debug, Clone)]
pub struct GetRecentlyPlayedTracks {
    /// The maximum number of items to return. Default: 20. Minimum: 1. Maximum: 50.
    pub limit: Option<u8>,

    /// The Unix timestamp in milliseconds. Returns all items after (but not including) this cursor position.
    /// If after is specified, before must not be specified.
    pub timeframe: QueryRange,
}

impl Endpoint for GetRecentlyPlayedTracks {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "me/player/recently-played".into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let limit = self.limit.unwrap_or(20).clamp(1, 50);

        let mut params = QueryParams::default();
        params.push("limit", &limit);

        match self.timeframe {
            QueryRange::Before(time) => {
                params.push("before", &time);
            }
            QueryRange::After(time) => {
                params.push("after", &time);
            }
        }

        params
    }
}

impl From<QueryRange> for GetRecentlyPlayedTracks {
    fn from(timeframe: QueryRange) -> Self {
        Self {
            limit: None,
            timeframe,
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
    fn test_get_recently_played_tracks_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("me/player/recently-played")
            .add_query_params(&[("limit", "20")])
            .add_query_params(&[("before", "1733877079")])
            .build();
        let client = SingleTestClient::new_raw(endpoint, "");
        api::ignore(GetRecentlyPlayedTracks::from(QueryRange::Before(
            1733877079,
        )))
        .query(&client)
        .unwrap();
    }
}
