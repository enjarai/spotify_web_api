use crate::{api::prelude::*, model::FollowType};

/// Check to see if the current user is following one or more artists or other Spotify users.
#[derive(Debug, Clone)]
pub struct CheckIfUserFollowsArtistsOrUsers {
    /// The ID type.
    pub type_: FollowType,

    /// A list of the artist or the user [Spotify IDs](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids).
    /// A maximum of 50 IDs can be sent in one request.
    pub ids: Vec<String>,
}

impl Endpoint for CheckIfUserFollowsArtistsOrUsers {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "me/following/contains".into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();
        params.push("type", &self.type_);
        params.push("ids", &self.ids.join(","));
        params
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
            .build();

        let expected_response = [false, false, true];

        let client = SingleTestClient::new_json(endpoint, &expected_response);

        let endpoint = CheckIfUserFollowsArtistsOrUsers {
            type_: FollowType::Artist,
            ids: vec![
                "2CIMQHirSU0MQqyYHq0eOx".to_owned(),
                "57dN52uHvrHOxijzpIgu3E".to_owned(),
                "1vCWHaC5f2uS3yhpwWbIA6".to_owned(),
            ],
        };

        let result: Vec<bool> = endpoint.query(&client).unwrap();

        assert_eq!(result, expected_response);
    }
}
