use spotify_web_api::{
    api::{endpoints::ArtistEndpoint, Query as _},
    model::Artist,
    Spotify,
};
use std::env;

fn main() -> anyhow::Result<()> {
    let client_id = env::var("SPOTIFY_CLIENT_ID")?;
    let client_secret = env::var("SPOTIFY_CLIENT_SECRET")?;

    let mut spotify = Spotify::with_client_credentials(client_id, client_secret)?;

    spotify.request_token()?;

    let artist: Artist = ArtistEndpoint::from("0559tR6WyukLWH68JIGBuC").query(&spotify)?;

    println!("\n{} -> {}\n", artist.name, artist.external_urls.spotify);

    Ok(())
}
