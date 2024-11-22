use super::{
    ExternalIds, ExternalUrls, ItemType, Market, Restrictions, SimplifiedAlbum, SimplifiedArtist,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkedFrom {
    /// Known external URLs for this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_urls: Option<ExternalUrls>,

    /// A link to the Web API endpoint providing full details of the item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,

    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The object type.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<ItemType>,

    /// The [Spotify URI](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    /// The album on which the track appears. The album object includes a link in href to full information about the album.
    pub album: SimplifiedAlbum,

    /// The artists who performed the track. Each artist object includes a link in href to more detailed information about the artist.
    pub artists: Vec<SimplifiedArtist>,

    /// A list of the countries in which the track can be played, identified by their ISO 3166-1 alpha-2 code.
    pub available_markets: Vec<Market>,

    /// The disc number (usually 1 unless the album consists of more than one disc).
    pub disc_number: u8,

    /// The track length in milliseconds.
    pub duration_ms: u32,

    /// Whether or not the track has explicit lyrics ( true = yes it does; false = no it does not OR unknown).
    pub explicit: bool,

    /// Known external IDs for the track.
    pub external_ids: ExternalIds,

    /// Known external URLs for this track.
    pub external_urls: ExternalUrls,

    /// A link to the Web API endpoint providing full details of the track.
    pub href: String,

    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the track.
    pub id: String,

    /// Part of the response when Track Relinking is applied. If true, the track is playable in the given market. Otherwise false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_playable: Option<bool>,

    /// Part of the response when Track Relinking is applied, and the requested track has been replaced with different track.
    /// The track in the `linked_from` object contains information about the originally requested track.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_from: Option<LinkedFrom>,

    /// Included in the response when a content restriction is applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<Restrictions>,

    /// The name of the track.
    pub name: String,

    /// The popularity of a track is a value between 0 and 100, with 100 being the most popular.
    pub popularity: u8,

    /// A link to a 30 second preview (MP3 format) of the track.
    pub preview_url: Option<String>,

    /// The number of the track. If an album has several discs, the track number is the number on the specified disc.
    pub track_number: u32,

    /// The object type.
    ///
    /// Allowed values: "track"
    #[serde(rename = "type")]
    pub type_: ItemType,

    /// The [Spotify URI](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the track.
    pub uri: String,

    /// Whether or not the track is from a local file.
    pub is_local: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimplifiedTrack {
    /// The album on which the track appears. The album object includes a link in href to full information about the album.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album: Option<SimplifiedAlbum>,

    /// The artists who performed the track. Each artist object includes a link in href to more detailed information about the artist.
    pub artists: Vec<SimplifiedArtist>,

    /// A list of the countries in which the track can be played, identified by their ISO 3166-1 alpha-2 code.
    pub available_markets: Vec<Market>,

    /// The disc number (usually 1 unless the album consists of more than one disc).
    pub disc_number: u8,

    /// The track length in milliseconds.
    pub duration_ms: u32,

    /// Whether or not the track has explicit lyrics ( true = yes it does; false = no it does not OR unknown).
    pub explicit: bool,

    /// Known external URLs for this track.
    pub external_urls: ExternalUrls,

    /// A link to the Web API endpoint providing full details of the track.
    pub href: String,

    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the track.
    pub id: String,

    /// Part of the response when Track Relinking is applied. If true, the track is playable in the given market. Otherwise false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_playable: Option<bool>,

    /// Part of the response when Track Relinking is applied, and the requested track has been replaced with different track.
    /// The track in the `linked_from` object contains information about the originally requested track.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_from: Option<LinkedFrom>,

    /// Included in the response when a content restriction is applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<Restrictions>,

    /// The name of the track.
    pub name: String,

    /// A link to a 30 second preview (MP3 format) of the track.
    pub preview_url: Option<String>,

    /// The number of the track. If an album has several discs, the track number is the number on the specified disc.
    pub track_number: u32,

    /// The object type.
    ///
    /// Allowed values: "track"
    #[serde(rename = "type")]
    pub type_: ItemType,

    /// The [Spotify URI](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the track.
    pub uri: String,

    /// Whether or not the track is from a local file.
    pub is_local: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedTrack {
    /// The date and time the track was saved.
    /// Timestamps are returned in ISO 8601 format as Coordinated Universal Time (UTC) with a zero offset: YYYY-MM-DDTHH:MM:SSZ.
    /// If the time is imprecise (for example, the date/time of an album release), an additional field indicates the precision;
    /// see for example, `release_date` in an album object.
    pub added_at: String,

    /// Information about the track.
    pub track: Track,
}

/// Spotify catalog information for several tracks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tracks {
    pub tracks: Vec<Option<Track>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackReference {
    /// A link to the Web API endpoint where full details of the playlist's tracks can be retrieved.
    pub href: String,

    /// Number of tracks in the playlist.
    pub total: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum SeedType {
    Artist,
    Track,
    Genre,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecommendationSeed {
    /// The number of tracks available after min_* and max_* filters have been applied.
    pub after_filtering_size: u32,

    /// The number of tracks available after relinking for regional availability.
    pub after_relinking_size: u32,

    /// A link to the full track or artist data for this seed. For tracks this will be a link to a Track Object.
    /// For artists a link to an Artist Object. For genre seeds, this value will be None.
    pub href: Option<String>,

    /// The id used to select this seed.
    /// This will be the same as the string used in the `seed_artists`, `seed_tracks` or `seed_genres` parameter.
    pub id: String,

    /// The number of recommended tracks available for this seed.
    pub initial_pool_size: usize,

    /// The entity type of this seed. One of artist, track or genre.
    #[serde(rename = "type")]
    pub type_: SeedType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendations {
    pub seeds: Vec<RecommendationSeed>,
    pub tracks: Vec<SimplifiedTrack>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn track() {
        let json = r#"
        {
			"album": {
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
			},
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
			],
			"available_markets": ["CA", "BR", "IT"],
			"disc_number": 0,
			"duration_ms": 0,
			"explicit": false,
			"external_ids": {
				"isrc": "string",
				"ean": "string",
				"upc": "string"
			},
			"external_urls": {
				"spotify": "string"
			},
			"href": "string",
			"id": "string",
			"is_playable": false,
			"linked_from": {},
			"restrictions": {
				"reason": "string"
			},
			"name": "string",
			"popularity": 0,
			"preview_url": "string",
			"track_number": 0,
			"type": "track",
			"uri": "string",
			"is_local": false
        }
        "#;

        crate::test::assert_deserialized!(Track, json);
    }

    #[test]
    fn recommendations() {
        let json = r#"
            {
				"tracks": [
					{
						"album": {
							"album_type": "ALBUM",
							"artists": [
								{
									"external_urls": {
										"spotify": "https://open.spotify.com/artist/0559tR6WyukLWH68JIGBuC"
									},
									"href": "https://api.spotify.com/v1/artists/0559tR6WyukLWH68JIGBuC",
									"id": "0559tR6WyukLWH68JIGBuC",
									"name": "Matt Elliott",
									"type": "artist",
									"uri": "spotify:artist:0559tR6WyukLWH68JIGBuC"
								}
							],
							"available_markets": ["US"],
							"external_urls": {
								"spotify": "https://open.spotify.com/album/7q8cIAINVLn24Sj4TNskDS"
							},
							"href": "https://api.spotify.com/v1/albums/7q8cIAINVLn24Sj4TNskDS",
							"id": "7q8cIAINVLn24Sj4TNskDS",
							"images": [
								{
									"height": 640,
									"url": "https://i.scdn.co/image/ab67616d0000b273b9f6afda6ed8c51f917568ec",
									"width": 640
								},
								{
									"height": 300,
									"url": "https://i.scdn.co/image/ab67616d00001e02b9f6afda6ed8c51f917568ec",
									"width": 300
								},
								{
									"height": 64,
									"url": "https://i.scdn.co/image/ab67616d00004851b9f6afda6ed8c51f917568ec",
									"width": 64
								}
							],
							"name": "Howling Songs",
							"release_date": "2008-03-11",
							"release_date_precision": "day",
							"total_tracks": 9,
							"type": "album",
							"uri": "spotify:album:7q8cIAINVLn24Sj4TNskDS"
						},
						"artists": [
							{
								"external_urls": {
									"spotify": "https://open.spotify.com/artist/0559tR6WyukLWH68JIGBuC"
								},
								"href": "https://api.spotify.com/v1/artists/0559tR6WyukLWH68JIGBuC",
								"id": "0559tR6WyukLWH68JIGBuC",
								"name": "Matt Elliott",
								"type": "artist",
								"uri": "spotify:artist:0559tR6WyukLWH68JIGBuC"
							}
						],
						"available_markets": ["US"],
						"disc_number": 1,
						"duration_ms": 392200,
						"explicit": false,
						"external_ids": {
							"isrc": "FR24Q0821006"
						},
						"external_urls": {
							"spotify": "https://open.spotify.com/track/4pXcwFJW1XOS9osVImkRpu"
						},
						"href": "https://api.spotify.com/v1/tracks/4pXcwFJW1XOS9osVImkRpu",
						"id": "4pXcwFJW1XOS9osVImkRpu",
						"is_local": false,
						"name": "I Name This Ship The Tragedy, Bless Her & All Who Sail With Her",
						"popularity": 18,
						"preview_url": "https://p.scdn.co/mp3-preview/dd4c72ff672d271d118798fee387add2cf2f4e8d?cid=cfe923b2d660439caf2b557b21f31221",
						"track_number": 6,
						"type": "track",
						"uri": "spotify:track:4pXcwFJW1XOS9osVImkRpu"
					}
				],
				"seeds": [
					{
						"initialPoolSize": 500,
						"afterFilteringSize": 500,
						"afterRelinkingSize": 500,
						"id": "0559tR6WyukLWH68JIGBuC",
						"type": "ARTIST",
						"href": "https://api.spotify.com/v1/artists/0559tR6WyukLWH68JIGBuC"
					}
				]
            }
            "#;

        crate::test::assert_deserialized!(Recommendations, json);
    }
}
