use crate::{api::prelude::*, model::FollowType};

/// Check to see if the current user is following one or more artists or other Spotify users.
#[derive(Debug, Builder, Clone, Endpoint)]
#[endpoint(method = GET, path = "me/following/contains")]
pub struct CheckIfUserFollowsArtistsOrUsers {
    /// The ID type.
    #[builder(setter(into))]
    pub type_: FollowType,

    /// A list of the artist or the user [Spotify IDs](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids).
    /// A maximum of 50 IDs can be sent in one request.
    pub ids: Vec<String>,
}

#[allow(dead_code)]
impl CheckIfUserFollowsArtistsOrUsersBuilder {
    fn id(&mut self, id: impl Into<String>) -> &mut Self {
        self.ids.get_or_insert_with(Vec::new).push(id.into());
        self
    }
}

impl CheckIfUserFollowsArtistsOrUsers {
    pub fn builder() -> CheckIfUserFollowsArtistsOrUsersBuilder {
        CheckIfUserFollowsArtistsOrUsersBuilder::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        api::Query as _,
        test::client::{ExpectedUrl, SingleTestClient},
    };

    #[test]
    fn test_check_if_user_follows_artists_or_users_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("me/following/contains")
            .add_query_params(&[("type", "artist")])
            .add_query_params(&[(
                "ids",
                "2CIMQHirSU0MQqyYHq0eOx,57dN52uHvrHOxijzpIgu3E,1vCWHaC5f2uS3yhpwWbIA6",
            )])
            .build()
            .unwrap();

        let expected_response = [false, false, true];

        let client = SingleTestClient::new_json(endpoint, &expected_response);

        let endpoint = CheckIfUserFollowsArtistsOrUsers::builder()
            .type_(FollowType::Artist)
            .id("2CIMQHirSU0MQqyYHq0eOx")
            .id("57dN52uHvrHOxijzpIgu3E")
            .id("1vCWHaC5f2uS3yhpwWbIA6")
            .build()
            .unwrap();

        let result: Vec<bool> = endpoint.query(&client).unwrap();

        assert_eq!(result, expected_response);
    }
}
