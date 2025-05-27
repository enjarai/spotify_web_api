use crate::api::prelude::*;

/// Remove one or more albums from the current user's 'Your Music' library.
#[derive(Debug, Clone, Endpoint)]
#[endpoint(method = DELETE, path = "me/albums")]
pub struct RemoveUserSavedAlbums {
    /// A list of [Spotify IDs](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the albums.
    pub ids: Vec<String>,
}

impl<T, I> From<I> for RemoveUserSavedAlbums
where
    I: IntoIterator<Item = T>,
    T: Into<String>,
{
    fn from(ids: I) -> Self {
        Self {
            ids: ids.into_iter().map(Into::into).collect(),
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
    use http::Method;

    #[test]
    fn test_remove_user_saved_albums_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::DELETE)
            .endpoint("me/albums")
            .add_query_params(&[("ids", "7F50uh7oGitmAEScRKV6pD,27XW2QTeqZGOKlm2Dt0PvN")])
            .build();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint =
            RemoveUserSavedAlbums::from(["7F50uh7oGitmAEScRKV6pD", "27XW2QTeqZGOKlm2Dt0PvN"]);

        api::ignore(endpoint).query(&client).unwrap();
    }
}
