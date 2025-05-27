use crate::{
    api::{Endpoint, prelude::*},
    model::PlaylistItem,
};
use serde_json::json;

/// Either reorder or replace items in a playlist depending on the request's parameters.
/// To reorder items, include `range_start`, `insert_before`, `range_length` and `snapshot_id` in the request's body.
/// To replace items, include uris as either a query parameter or in the request's body.
/// Replacing items in a playlist will overwrite its existing items.
/// This operation can be used for replacing or clearing items in a playlist.
#[derive(Debug, Clone)]
pub struct UpdatePlaylistItems {
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) of the playlist.
    pub id: String,

    /// A list of [Spotify URIs](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) to set, can be track or episode URIs.
    pub uris: Option<Vec<PlaylistItem>>,

    /// The position of the first item to be reordered.
    pub range_start: u32,

    /// The position where the items should be inserted.
    pub insert_before: u32,

    /// The amount of items to be reordered. Defaults to 1 if not set.
    pub range_length: Option<usize>,

    /// The playlist's snapshot ID against which you want to make the changes.
    pub snapshot_id: Option<String>,
}

impl UpdatePlaylistItems {
    pub fn new(id: impl Into<String>, range_start: u32, insert_before: u32) -> Self {
        Self {
            id: id.into(),
            uris: None,
            range_start,
            insert_before,
            range_length: None,
            snapshot_id: None,
        }
    }

    pub fn uris(mut self, uris: Vec<PlaylistItem>) -> Self {
        self.uris = Some(uris);
        self
    }

    pub fn uri(mut self, uri: PlaylistItem) -> Self {
        self.uris.get_or_insert_with(Vec::new).push(uri);
        self
    }

    pub fn range_length(mut self, range_length: usize) -> Self {
        self.range_length = Some(range_length);
        self
    }

    pub fn snapshot_id(mut self, snapshot_id: impl Into<String>) -> Self {
        self.snapshot_id = Some(snapshot_id.into());
        self
    }
}

impl Endpoint for UpdatePlaylistItems {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("playlists/{}/tracks", self.id).into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();
        let uris: Option<String> = self.uris.as_ref().map(|uris| {
            uris.iter()
                .map(|uri| uri.to_string())
                .collect::<Vec<_>>()
                .join(",")
        });
        params.push_opt("uris", uris);
        params
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let range_length = self.range_length.unwrap_or(1);

        let mut body = json!({
            "range_start": self.range_start,
            "insert_before": self.insert_before,
            "range_length": range_length,
        });

        if let Some(snapshot_id) = self.snapshot_id.as_ref() {
            body["snapshot_id"] = json!(snapshot_id);
        }

        JsonParams::into_body(&body)
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
    fn test_update_playlist_items_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .content_type("application/json")
            .endpoint("playlists/3cEYpjA9oz9GiPac4AsH4n/tracks")
            .body_str(r#"{"insert_before":3,"range_length":2,"range_start":1}"#)
            .build();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = UpdatePlaylistItems::new("3cEYpjA9oz9GiPac4AsH4n", 1, 3).range_length(2);

        api::ignore(endpoint).query(&client).unwrap();
    }
}
