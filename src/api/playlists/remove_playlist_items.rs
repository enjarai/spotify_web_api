use crate::{
    api::{prelude::*, Endpoint},
    model::PlaylistItem,
};
use serde_json::json;

/// Remove one or more items from a user's playlist.
#[derive(Debug, Clone, Builder)]
pub struct RemovePlaylistItems {
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) of the playlist.
    #[builder(setter(into))]
    id: String,

    /// A list of [Spotify URIs](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) to set, can be track or episode URIs.
    tracks: Vec<PlaylistItem>,

    /// The playlist's snapshot ID against which you want to make the changes.
    /// The API will validate that the specified items exist and in the specified positions and make the changes, even if more recent changes have been made to the playlist.
    #[builder(setter(into))]
    snapshot_id: String,
}

impl RemovePlaylistItems {
    pub fn builder() -> RemovePlaylistItemsBuilder {
        RemovePlaylistItemsBuilder::default()
    }
}

impl RemovePlaylistItemsBuilder {
    pub fn track(&mut self, item: impl Into<PlaylistItem>) -> &mut Self {
        self.tracks.get_or_insert_with(Vec::new).push(item.into());
        self
    }
}

impl Endpoint for RemovePlaylistItems {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("playlists/{}/tracks", self.id).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let tracks = self
            .tracks
            .iter()
            .map(|item| {
                let mut obj = json!({});
                obj["uri"] = item.to_string().clone().into();
                obj
            })
            .collect::<Vec<_>>();

        let body = json!({
            "snapshot_id": self.snapshot_id,
            "tracks": tracks,
        });

        JsonParams::into_body(&body)
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
    fn test_remove_playlist_items_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::DELETE)
            .content_type("application/json")
            .endpoint("playlists/3cEYpjA9oz9GiPac4AsH4n/tracks")
            .body_str(r#"{"snapshot_id":"abc","tracks":[{"uri":"spotify:track:4iV5W9uYEdYUVa79Axb7Rh"},{"uri":"spotify:track:1301WleyT98MSxVHPZCA6M"}]}"#)
            .build()
            .unwrap();

        let client = SingleTestClient::new_raw(endpoint, "");

        let track_1 = TrackId::from_id("4iV5W9uYEdYUVa79Axb7Rh").unwrap();
        let track_2 = TrackId::from_id("1301WleyT98MSxVHPZCA6M").unwrap();

        let endpoint = RemovePlaylistItems::builder()
            .id("3cEYpjA9oz9GiPac4AsH4n")
            .snapshot_id("abc")
            .track(track_1)
            .track(track_2)
            .build()
            .unwrap();

        api::ignore(endpoint).query(&client).unwrap();
    }
}
