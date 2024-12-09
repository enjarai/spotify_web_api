use crate::{api::prelude::*, model::FollowType};

/// Add the current user as a follower of one or more artists or other Spotify users.
#[derive(Debug, Builder, Clone, Endpoint)]
#[endpoint(method = PUT, path = "me/following")]
pub struct FollowArtistsOrUsers {
    /// The ID type.
    pub type_: FollowType,

    /// A list of the artist or the user [Spotify IDs](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids).
    /// A maximum of 50 IDs can be sent in one request.
    #[endpoint(body)]
    pub ids: Vec<String>,
}

impl FollowArtistsOrUsersBuilder {
    pub fn id(&mut self, id: impl Into<String>) -> &mut Self {
        self.ids.get_or_insert_with(Vec::new).push(id.into());
        self
    }
}

impl FollowArtistsOrUsers {
    pub fn builder() -> FollowArtistsOrUsersBuilder {
        FollowArtistsOrUsersBuilder::default()
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
    fn test_follow_artists_or_users_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .content_type("application/json")
            .endpoint("me/following")
            .add_query_params(&[("type", "artist")])
            .body_str(r#"{"ids":["2CIMQHirSU0MQqyYHq0eOx","57dN52uHvrHOxijzpIgu3E","1vCWHaC5f2uS3yhpwWbIA6"]}"#)
            .build()
            .unwrap();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = FollowArtistsOrUsers::builder()
            .type_(FollowType::Artist)
            .id("2CIMQHirSU0MQqyYHq0eOx")
            .id("57dN52uHvrHOxijzpIgu3E")
            .id("1vCWHaC5f2uS3yhpwWbIA6")
            .build()
            .unwrap();

        api::ignore(endpoint).query(&client).unwrap();
    }
}
