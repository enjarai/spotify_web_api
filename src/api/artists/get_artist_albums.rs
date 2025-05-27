use crate::{
    api::{Endpoint, prelude::*},
    model::AlbumType,
};

/// Get Spotify catalog information about an artist's albums.
#[derive(Debug, Clone)]
pub struct GetArtistAlbums {
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the artist.
    pub id: String,

    /// A list of keywords that will be used to filter the response. If not supplied, all album types will be returned.
    pub include_groups: Option<Vec<AlbumType>>,

    /// An [ISO 3166-1 alpha-2 country code](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2).
    /// If a country code is specified, only content that is available in that market will be returned.
    /// If a valid user access token is specified in the request header, the country associated with the user account will take priority over this parameter.
    ///
    /// # Notes
    /// If neither market or user country are provided, the content is considered unavailable for the client.
    /// Users can view the country that is associated with their account in the [account settings](https://www.spotify.com/account/overview/).
    pub market: Option<Market>,
}

impl GetArtistAlbums {
    pub fn new(id: impl Into<String>) -> Self {
        Self::from(id)
    }

    pub fn include_group(mut self, include_group: AlbumType) -> Self {
        match &mut self.include_groups {
            Some(groups) => groups.push(include_group),
            None => self.include_groups = Some(vec![include_group]),
        }
        self
    }

    pub fn market(mut self, market: Market) -> Self {
        self.market = Some(market);
        self
    }
}

impl Endpoint for GetArtistAlbums {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("artists/{}/albums", self.id).into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();
        let include_groups_str = self.include_groups.as_ref().map(|ids| {
            ids.iter()
                .map(|id| id.snake_case())
                .collect::<Vec<_>>()
                .join(",")
        });

        params.push_opt("include_groups", include_groups_str);
        params.push_opt("market", self.market.as_ref());

        params
    }
}

impl Pageable for GetArtistAlbums {}

impl<T: Into<String>> From<T> for GetArtistAlbums {
    fn from(id: T) -> Self {
        Self {
            id: id.into(),
            market: None,
            include_groups: None,
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
    fn test_get_artist_albums_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("artists/0TnOYISbd1XYRBk9myaseg/albums")
            .add_query_params(&[("include_groups", "single,appears_on")])
            .build();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GetArtistAlbums::new("0TnOYISbd1XYRBk9myaseg")
            .include_group(AlbumType::Single)
            .include_group(AlbumType::AppearsOn);

        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn test_get_artist_albums_endpoint_with_no_include_groups() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("artists/0TnOYISbd1XYRBk9myaseg/albums")
            .build();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = GetArtistAlbums::new("0TnOYISbd1XYRBk9myaseg");

        api::ignore(endpoint).query(&client).unwrap();
    }
}
