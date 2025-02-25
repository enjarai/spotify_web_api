use spotify_web_api::{
    Spotify,
    api::{Query as _, artists::GetArtist},
    model::Artist,
};
use std::env;

fn main() -> anyhow::Result<()> {
    let client_id = env::var("SPOTIFY_CLIENT_ID")?;
    let client_secret = env::var("SPOTIFY_CLIENT_SECRET")?;

    let spotify = Spotify::with_client_credentials(client_id, client_secret)?;

    spotify.request_token()?;

    let artist: Artist = GetArtist::from("0559tR6WyukLWH68JIGBuC").query(&spotify)?;

    println!("\n{} -> {}\n", artist.name, artist.external_urls.spotify);

    Ok(())
}
