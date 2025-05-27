use crate::{api::prelude::*, model::PlaylistItem};

/// Add an item to the end of the user's current playback queue.
/// This API only works for users who have Spotify Premium.
/// The order of execution is not guaranteed when you use this API with other Player API endpoints.
#[derive(Debug, Clone, Endpoint)]
#[endpoint(method = POST, path = "me/player/queue")]
pub struct AddItemToPlaybackQueue {
    /// The id of the device this command is targeting. If not supplied, the user's currently active device is the target.
    pub device_id: Option<String>,

    /// The uri of the item to add to the queue. Must be a track or an episode uri.
    pub uri: PlaylistItem,
}

impl From<PlaylistItem> for AddItemToPlaybackQueue {
    fn from(uri: PlaylistItem) -> Self {
        Self {
            device_id: None,
            uri,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        api::{self, Query as _},
        model::TrackId,
        test::client::{ExpectedUrl, SingleTestClient},
    };

    #[test]
    fn test_add_item_to_playback_queue_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("me/player/queue")
            .add_query_params(&[("uri", "spotify:track:4iV5W9uYEdYUVa79Axb7Rh")])
            .build();
        let client = SingleTestClient::new_raw(endpoint, "");

        let item: PlaylistItem = TrackId::from_id("4iV5W9uYEdYUVa79Axb7Rh").unwrap().into();

        api::ignore(AddItemToPlaybackQueue::from(item))
            .query(&client)
            .unwrap();
    }
}
