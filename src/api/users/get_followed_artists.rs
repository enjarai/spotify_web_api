use crate::{api::prelude::*, model::FollowedArtistsType};

/// Get the current user's followed artists.
#[derive(Debug, Builder, Clone, Endpoint)]
#[endpoint(method = GET, path = "me/following")]
pub struct GetFollowedArtists {
    /// The ID type: currently only artist is supported.
    type_: FollowedArtistsType,

    /// The last artist ID retrieved from the previous request.
    #[builder(setter(into, strip_option), default)]
    after: Option<String>,
}

impl GetFollowedArtists {
    pub fn builder() -> GetFollowedArtistsBuilder {
        GetFollowedArtistsBuilder::default()
    }

    pub fn with_after(after: Option<impl Into<String>>) -> Self {
        Self {
            type_: FollowedArtistsType::Artist,
            after: after.map(Into::into),
        }
    }
}

impl Default for GetFollowedArtists {
    fn default() -> Self {
        Self {
            type_: FollowedArtistsType::Artist,
            after: None,
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
    fn test_get_followed_artists_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("me/following")
            .add_query_params(&[("type", "artist")])
            .build()
            .unwrap();

        let client = SingleTestClient::new_raw(endpoint, "");

        api::ignore(GetFollowedArtists::default())
            .query(&client)
            .unwrap();
    }

    #[test]
    fn test_get_followed_artists_endpoint_with_after() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("me/following")
            .add_query_params(&[("type", "artist")])
            .add_query_params(&[("after", "2CIMQHirSU0MQqyYHq0eOx")])
            .build()
            .unwrap();

        let client = SingleTestClient::new_raw(endpoint, "");

        api::ignore(GetFollowedArtists::with_after(Some(
            "2CIMQHirSU0MQqyYHq0eOx",
        )))
        .query(&client)
        .unwrap();
    }
}
