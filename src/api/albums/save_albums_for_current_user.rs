use crate::api::prelude::*;

/// Save one or more albums to the current user's 'Your Music' library.
#[derive(Debug, Clone, Endpoint)]
#[endpoint(method = PUT, path = "me/albums")]
pub struct SaveAlbumsforCurrentUser {
    /// A list of [Spotify IDs](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the albums.
    pub ids: Vec<String>,
}

impl<T, I> From<I> for SaveAlbumsforCurrentUser
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
    fn test_save_albums_for_current_user_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .endpoint("me/albums")
            .add_query_params(&[("ids", "7F50uh7oGitmAEScRKV6pD,27XW2QTeqZGOKlm2Dt0PvN")])
            .build();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint =
            SaveAlbumsforCurrentUser::from(["7F50uh7oGitmAEScRKV6pD", "27XW2QTeqZGOKlm2Dt0PvN"]);

        api::ignore(endpoint).query(&client).unwrap();
    }
}
