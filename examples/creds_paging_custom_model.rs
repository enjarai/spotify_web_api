use spotify_web_api::{
    api::{self, albums::GetAlbumTracks, Query as _},
    Spotify,
};
use std::env;

#[derive(serde::Deserialize)]
struct Track {
    name: String,
}

fn main() -> anyhow::Result<()> {
    let client_id = env::var("SPOTIFY_CLIENT_ID")?;
    let client_secret = env::var("SPOTIFY_CLIENT_SECRET")?;

    let spotify = Spotify::with_client_credentials(client_id, client_secret)?;

    spotify.request_token()?;

    let tracks: Vec<Track> =
        api::paged_with_limit(GetAlbumTracks::from("7F50uh7oGitmAEScRKV6pD"), 6).query(&spotify)?;

    for track in tracks {
        println!("{}", track.name);
    }

    Ok(())
}
