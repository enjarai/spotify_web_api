use http::Method;
use spotify_web_api::{
    Spotify,
    api::{Endpoint, Query as _},
};
use std::{borrow::Cow, env};

#[derive(serde::Deserialize)]
struct Track {
    name: String,
}

struct TrackID<'a>(&'a str);

impl Endpoint for TrackID<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("tracks/{}", self.0).into()
    }
}

fn main() -> anyhow::Result<()> {
    let client_id = env::var("SPOTIFY_CLIENT_ID")?;
    let client_secret = env::var("SPOTIFY_CLIENT_SECRET")?;

    let spotify = Spotify::with_client_credentials(client_id, client_secret)?;

    spotify.request_token()?;

    let track: Track = TrackID("4PTG3Z6ehGkBFwjybzWkR8").query(&spotify)?;

    println!("\n{}\n", track.name);

    Ok(())
}
