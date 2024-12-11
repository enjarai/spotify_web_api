use crate::api::{prelude::*, Endpoint};
use serde_json::json;

/// Change a playlist's name and public/private state. (The user must, of course, own the playlist.)
#[derive(Debug, Builder, Clone)]
pub struct ChangePlaylistDetails {
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) of the playlist.
    #[builder(setter(into))]
    id: String,

    /// The new name for the playlist, for example "My New Playlist Title".
    #[builder(setter(into, strip_option), default)]
    name: Option<String>,

    /// The playlist's public/private status (if it should be added to the user's profile or not):
    /// true the playlist will be public,
    /// false the playlist will be private, null the playlist status is not relevant.
    /// For more about public/private status, see [Working with Playlists](https://developer.spotify.com/documentation/web-api/concepts/playlists).
    #[builder(setter(strip_option), default)]
    public: Option<bool>,

    /// If true, the playlist will become collaborative and other users will be able to modify the playlist in their Spotify client.
    /// # Note:
    /// You can only set collaborative to true on non-public playlists.
    #[builder(setter(strip_option), default)]
    collaborative: Option<bool>,

    /// Value for playlist description as displayed in Spotify Clients and in the Web API.
    #[builder(setter(into, strip_option), default)]
    description: Option<String>,
}

impl ChangePlaylistDetails {
    pub fn builder() -> ChangePlaylistDetailsBuilder {
        ChangePlaylistDetailsBuilder::default()
    }
}

impl Endpoint for ChangePlaylistDetails {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("playlists/{}", self.id).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut body = json!({});

        if let Some(name) = &self.name {
            body["name"] = json!(name);
        }

        if let Some(description) = &self.description {
            body["description"] = json!(description);
        }

        if let Some(public) = self.public {
            body["public"] = json!(public);
        }

        if let Some(collaborative) = self.collaborative {
            body["collaborative"] = json!(collaborative);
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
    fn test_change_playlist_details_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .content_type("application/json")
            .endpoint("playlists/3cEYpjA9oz9GiPac4AsH4n")
            .body_str(r#"{"description":"New Playlist Description","name":"Updated Playlist Name","public":false}"#)
            .build()
            .unwrap();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ChangePlaylistDetails::builder()
            .id("3cEYpjA9oz9GiPac4AsH4n")
            .name("Updated Playlist Name")
            .description("New Playlist Description")
            .public(false)
            .build()
            .unwrap();

        api::ignore(endpoint).query(&client).unwrap();
    }
}
