use crate::api::prelude::*;

/// Get a list of new album releases featured in Spotify (shown, for example, on a Spotify player’s “Browse” tab).
#[derive(Default, Debug, Clone, Endpoint)]
#[endpoint(
	method = GET,
	path = "browse/new-releases",
)]
pub struct GetNewReleases;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        api::Query as _,
        model::NewReleases,
        test::client::{ExpectedUrl, SingleTestClient},
    };

    const RESPONSE: &str = r#"
    	{
			"albums": {
				"href": "https://api.spotify.com/v1/me/shows?offset=0&limit=20",
				"limit": 20,
				"next": null,
				"offset": 0,
				"previous": null,
				"total": 1,
				"items": [
					{
						"album_type": "compilation",
						"total_tracks": 9,
						"available_markets": ["CA", "BR", "IT"],
						"external_urls": {
							"spotify": "string"
						},
						"href": "string",
						"id": "2up3OPMp9Tb4dAKM2erWXQ",
						"images": [
							{
								"url": "https://i.scdn.co/image/ab67616d00001e02ff9ca10b55ce82ae553c8228",
								"height": 300,
								"width": 300
							}
						],
						"name": "string",
						"release_date": "1981-12",
						"release_date_precision": "year",
						"restrictions": {
							"reason": "market"
						},
						"type": "album",
						"uri": "spotify:album:2up3OPMp9Tb4dAKM2erWXQ",
						"artists": [
							{
								"external_urls": {
									"spotify": "string"
								},
								"href": "string",
								"id": "string",
								"name": "string",
								"type": "artist",
								"uri": "string"
							}
						]
					}
				]
			}
    	}
     	"#;

    #[test]
    fn test_get_new_releases_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("browse/new-releases")
            .build();

        let client = SingleTestClient::new_raw(endpoint, RESPONSE);

        let new_releases: NewReleases = GetNewReleases.query(&client).unwrap();

        assert!(new_releases.albums.total == 1);
    }
}
