use spotify_web_api::{
    api::{tracks::GetTrack, Query as _},
    Spotify,
};
use std::env;

#[derive(serde::Deserialize)]
struct Track {
    name: String,
    artists: Vec<Artist>,
}

#[derive(serde::Deserialize)]
struct Artist {
    name: String,
}

fn main() -> anyhow::Result<()> {
    let client_id = env::var("SPOTIFY_CLIENT_ID")?;
    let client_secret = env::var("SPOTIFY_CLIENT_SECRET")?;

    let spotify = Spotify::with_client_credentials(client_id, client_secret)?;

    spotify.request_token()?;

    let track: Track = GetTrack::from("2HZasNmIAzprwJjFLPoSGM").query(&spotify)?;

    let artist_names = track
        .artists
        .iter()
        .map(|artist| artist.name.as_str())
        .collect::<Vec<&str>>()
        .join(", ");

    println!("\n{} - {}\n", track.name, artist_names);

    Ok(())
}
