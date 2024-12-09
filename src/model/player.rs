use super::{ContextType, Cursors, ExternalUrls, ItemType, Track, TrackItem};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    /// The device ID. This ID is unique and persistent to some extent.
    /// However, this is not guaranteed and any cached `device_id` should periodically be cleared out and refetched as necessary.
    pub id: Option<String>,

    /// If this device is the currently active device.
    pub is_active: bool,

    /// If this device is currently in a private session.
    pub is_private_session: bool,

    /// Whether controlling this device is restricted.
    /// At present if this is "true" then no Web API commands will be accepted by this device.
    pub is_restricted: bool,

    /// A human-readable name for the device. Some devices have a name that the user can configure (e.g. "Loudest speaker")
    /// and some devices have a generic name associated with the manufacturer or device model.
    pub name: String,

    #[serde(rename = "type")]
    /// Device type, such as "computer", "smartphone" or "speaker".
    pub type_: String,

    /// The current volume in percent.
    pub volume_percent: Option<u8>,

    /// If this device can be used to set the volume.
    pub supports_volume: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Devices {
    pub devices: Vec<Option<Device>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RepeatState {
    Track,
    Context,
    Off,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    /// The object type.
    #[serde(rename = "type")]
    pub type_: ItemType,

    /// A link to the Web API endpoint providing full details of the track.
    pub href: Option<String>,

    /// External URLs for this context.
    pub external_urls: ExternalUrls,

    /// The Spotify URI for the context.
    pub uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CurrentlyPlayingType {
    Track,
    Episode,
    Ad,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybackState {
    /// The device that is currently active.
    pub device: Device,

    /// off, track, context
    pub repeat_state: RepeatState,

    /// If shuffle is on or off.
    pub shuffle_state: bool,

    /// The context object.
    pub context: Option<Context>,

    /// Unix Millisecond Timestamp when data was fetched.
    pub timestamp: Option<i64>,

    /// Progress into the currently playing track or episode.
    pub progress_ms: Option<u32>,

    /// If something is currently playing, return true.
    pub is_playing: bool,

    /// The currently playing track or episode.
    pub item: Option<TrackItem>,

    /// The object type of the currently playing item. Can be one of track, episode, ad or unknown.
    pub currently_playing_type: CurrentlyPlayingType,

    /// Allows to update the user interface based on which playback actions are available within the current context.
    pub actions: Actions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interrupting_playback: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pausing: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resuming: Option<bool>,

    /// Seeking playback location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seeking: Option<bool>,

    /// Skipping to the next context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipping_next: Option<bool>,

    /// Skipping to the previous context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipping_prev: Option<bool>,

    /// Toggling repeat context flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toggling_repeat_context: Option<bool>,

    /// Toggling shuffle flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toggling_shuffle: Option<bool>,

    /// Toggling repeat track flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toggling_repeat_track: Option<bool>,

    /// Transfering playback between devices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transferring_playback: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentlyPlaying {
    /// The context object.
    pub context: Option<Context>,

    /// Unix Millisecond Timestamp when data was fetched.
    pub timestamp: Option<i64>,

    /// Progress into the currently playing track or episode.
    pub progress_ms: Option<u32>,

    /// If something is currently playing, return true.
    pub is_playing: bool,

    /// The currently playing track or episode.
    pub item: Option<TrackItem>,

    /// The object type of the currently playing item. Can be one of track, episode, ad or unknown.
    pub currently_playing_type: CurrentlyPlayingType,

    /// Allows to update the user interface based on which playback actions are available within the current context.
    pub actions: Actions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayHistory {
    /// The track the user listened to.
    pub track: Track,

    /// The date and time the track was played.
    pub played_at: String,

    /// The context the track was played from.
    pub context: Context,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentlyPlayedTracks {
    /// A link to the Web API endpoint returning the full result of the request.
    pub href: String,

    /// The maximum number of items in the response (as set in the query or by default).
    pub limit: usize,

    /// URL to the next page of items.
    pub next: Option<String>,

    /// The cursors used to find the next set of items.
    pub cursors: Option<Cursors>,

    /// The total number of items available to return.
    pub total: Option<usize>,

    pub items: Vec<PlayHistory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Queue {
    /// The currently playing track or episode.
    pub currently_playing: Option<TrackItem>,

    /// The tracks or episodes in the queue.
    pub queue: Vec<TrackItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Offset {
    Position(u32),
    Uri(ContextType),
}

impl From<u32> for Offset {
    fn from(position: u32) -> Self {
        Self::Position(position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn playback_state() {
        let json = r#"
        {
			"device": {
				"id": "string",
				"is_active": false,
				"is_private_session": false,
				"is_restricted": false,
				"name": "Kitchen speaker",
				"type": "computer",
				"volume_percent": 59,
				"supports_volume": false
			},
			"repeat_state": "off",
			"shuffle_state": false,
			"context": {
				"type": "track",
				"href": "string",
				"external_urls": {
					"spotify": "string"
				},
				"uri": "string"
			},
			"timestamp": 0,
			"progress_ms": 0,
			"is_playing": false,
			"item": {
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
				],
				"available_markets": ["US"],
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
			},
			"currently_playing_type": "unknown",
			"actions": {
				"interrupting_playback": false,
				"pausing": false,
				"resuming": false,
				"seeking": false,
				"skipping_next": false,
				"skipping_prev": false,
				"toggling_repeat_context": false,
				"toggling_shuffle": false,
				"toggling_repeat_track": false,
				"transferring_playback": false
			}
        }
        "#;

        crate::test::assert_deserialized!(PlaybackState, json);
    }
}
