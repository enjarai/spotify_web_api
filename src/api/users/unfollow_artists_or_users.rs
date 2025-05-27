use crate::{api::prelude::*, model::FollowType};

/// Remove the current user as a follower of one or more artists or other Spotify users.
#[derive(Debug, Clone, Endpoint)]
#[endpoint(method = DELETE, path = "me/following")]
pub struct UnfollowArtistsOrUsers {
    /// The ID type.
    pub type_: FollowType,

    /// A list of the artist or the user [Spotify IDs](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids).
    /// A maximum of 50 IDs can be sent in one request.
    #[endpoint(body)]
    pub ids: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        api::{self, Query as _},
        test::client::{ExpectedUrl, SingleTestClient},
    };

    #[test]
    fn test_unfollow_artists_or_users_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::DELETE)
            .content_type("application/json")
            .endpoint("me/following")
            .add_query_params(&[("type", "artist")])
            .body_str(r#"{"ids":["2CIMQHirSU0MQqyYHq0eOx","57dN52uHvrHOxijzpIgu3E","1vCWHaC5f2uS3yhpwWbIA6"]}"#)
            .build();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = UnfollowArtistsOrUsers {
            type_: FollowType::Artist,
            ids: vec![
                "2CIMQHirSU0MQqyYHq0eOx".to_owned(),
                "57dN52uHvrHOxijzpIgu3E".to_owned(),
                "1vCWHaC5f2uS3yhpwWbIA6".to_owned(),
            ],
        };

        api::ignore(endpoint).query(&client).unwrap();
    }
}
