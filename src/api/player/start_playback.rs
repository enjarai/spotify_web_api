use crate::{
    api::{prelude::*, Endpoint},
    model::{ContextType, Offset, TrackId},
};

/// Start a new context or resume current playback on the user's active device.
/// This API only works for users who have Spotify Premium.
/// The order of execution is not guaranteed when you use this API with other Player API endpoints.
#[derive(Debug, Builder, Clone)]
pub struct StartPlayback {
    /// The id of the device this command is targeting. If not supplied, the user's currently active device is the target.
    #[builder(setter(into, strip_option), default)]
    device_id: Option<String>,

    /// Spotify URI of the context to play.
    #[builder(setter(into, strip_option), default)]
    context_uri: Option<ContextType>,

    /// Spotify track URIs to play.
    #[builder(setter(strip_option), default)]
    uris: Option<Vec<TrackId>>,

    /// Indicates from where in the context playback should start.
    #[builder(setter(into, strip_option), default)]
    offset: Option<Offset>,

    /// Indicates from what position to start playback.
    position_ms: u32,
}

impl StartPlaybackBuilder {
    pub fn uri(&mut self, uri: TrackId) -> &mut Self {
        match self.uris {
            Some(ref mut uris) => uris.get_or_insert_with(Vec::new).push(uri),
            None => {
                self.uris = Some(Some(vec![uri]));
            }
        }
        self
    }
}

impl StartPlayback {
    pub fn builder() -> StartPlaybackBuilder {
        StartPlaybackBuilder::default()
    }
}

impl<T: Into<String>> From<T> for StartPlayback {
    fn from(device_id: T) -> Self {
        Self {
            device_id: Some(device_id.into()),
            context_uri: None,
            uris: None,
            offset: None,
            position_ms: 0,
        }
    }
}

impl Endpoint for StartPlayback {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "me/player/play".into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();
        params.push_opt("device_id", self.device_id.as_ref());
        params
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut body = serde_json::json!({});

        if let Some(context_uri) = self.context_uri.as_ref() {
            body["context_uri"] = context_uri.uri().into();
        }

        if let Some(uris) = &self.uris {
            body["uris"] = uris.iter().map(|uri| uri.uri()).collect::<Vec<_>>().into();
        }

        if let Some(offset) = self.offset.as_ref() {
            body["offset"] = match offset {
                Offset::Position(pos) => {
                    serde_json::json!({ "position": pos })
                }
                Offset::Uri(context) => serde_json::json!({ "uri": context.uri() }),
            }
        }

        body["position_ms"] = self.position_ms.into();

        JsonParams::into_body(&body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        api::{self, Query as _},
        model::AlbumId,
        test::client::{ExpectedUrl, SingleTestClient},
    };

    #[test]
    fn test_start_playback_endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .content_type("application/json")
            .endpoint("me/player/play")
            .add_query_params(&[("device_id", "xxxxxxxxxxxxxxxxxxxxxx")])
            .body_str(r#"{"position_ms":0}"#)
            .build()
            .unwrap();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = StartPlayback::from("xxxxxxxxxxxxxxxxxxxxxx");

        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn test_start_playback_endpoint_with_context_and_offset() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::PUT)
            .content_type("application/json")
            .endpoint("me/player/play")
            .add_query_params(&[("device_id", "xxxxxxxxxxxxxxxxxxxxxx")])
            .body_str(r#"{"context_uri":"spotify:album:5ht7ItJgpBH7W6vJ5BqpPr","offset":{"position":5},"position_ms":0}"#)
            .build()
            .unwrap();

        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = StartPlayback::builder()
            .context_uri(ContextType::Album(
                AlbumId::from_id("5ht7ItJgpBH7W6vJ5BqpPr").unwrap(),
            ))
            .offset(Offset::Position(5))
            .position_ms(0)
            .device_id("xxxxxxxxxxxxxxxxxxxxxx")
            .build()
            .unwrap();

        api::ignore(endpoint).query(&client).unwrap();
    }
}
