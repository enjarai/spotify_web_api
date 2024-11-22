use super::{Cursors, ExternalUrls, Followers, Image, ItemType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artist {
    /// Known external URLs for this artist.
    pub external_urls: ExternalUrls,

    /// Information about the followers of the artist.
    pub followers: Followers,

    /// A list of the genres the artist is associated with. If not yet classified, the array is empty.
    pub genres: Vec<String>,

    /// A link to the Web API endpoint providing full details of the artist.
    pub href: String,

    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the artist.
    pub id: String,

    /// Images of the artist in various sizes, widest first.
    pub images: Vec<Image>,

    /// The name of the artist.
    pub name: String,

    /// The popularity of the artist. The value will be between 0 and 100, with 100 being the most popular.
    /// The artist's popularity is calculated from the popularity of all the artist's tracks.
    pub popularity: u8,

    /// The object type.
    ///
    /// Allowed values: "artist"
    #[serde(rename = "type")]
    pub type_: ItemType,

    /// The [Spotify URI](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the artist.
    pub uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimplifiedArtist {
    /// Known external URLs for this artist.
    pub external_urls: ExternalUrls,

    /// A link to the Web API endpoint providing full details of the artist.
    pub href: String,

    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the artist.
    pub id: String,

    /// The name of the artist.
    pub name: String,

    /// The object type.
    ///
    /// Allowed values: "artist"
    #[serde(rename = "type")]
    pub type_: ItemType,

    /// The [Spotify URI](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the artist.
    pub uri: String,
}

/// Spotify catalog information for several artists
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artists {
    pub artists: Vec<Option<Artist>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FollowedArtist {
    /// A link to the Web API endpoint returning the full result of the request.
    pub href: String,

    /// The maximum number of items in the response (as set in the query or by default).
    pub limit: usize,

    /// URL to the next page of items.
    pub next: Option<String>,

    /// The cursors used to find the next set of items.
    pub cursors: Cursors,

    /// The total number of items available to return.
    pub total: usize,

    pub items: Vec<Artist>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FollowedArtists {
    pub artists: FollowedArtist,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn artist() {
        let json = r#"
       	{
			"external_urls": {
				"spotify": "string"
			},
			"followers": {
				"href": "string",
				"total": 0
			},
			"genres": ["Prog rock", "Grunge"],
			"href": "string",
			"id": "string",
			"images": [
				{
					"url": "https://i.scdn.co/image/ab67616d00001e02ff9ca10b55ce82ae553c8228",
					"height": 300,
					"width": 300
				}
			],
			"name": "string",
			"popularity": 0,
			"type": "artist",
			"uri": "string"
        }
        "#;

        crate::test::assert_deserialized!(Artist, json);
    }

    #[test]
    fn simplified_artist() {
        let json = r#"
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
        "#;

        crate::test::assert_deserialized!(SimplifiedArtist, json);
    }
}
