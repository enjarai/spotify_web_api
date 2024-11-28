use spotify_web_api::{
    api::{artists::GetArtist, AsyncQuery as _},
    model::Artist,
    AsyncSpotify,
};
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client_id = env::var("SPOTIFY_CLIENT_ID")?;
    let client_secret = env::var("SPOTIFY_CLIENT_SECRET")?;

    let spotify = AsyncSpotify::with_client_credentials(client_id, client_secret)?;

    spotify.request_token().await?;

    let artist: Artist = GetArtist::from("0559tR6WyukLWH68JIGBuC")
        .query_async(&spotify)
        .await?;

    println!("\n{} -> {}\n", artist.name, artist.external_urls.spotify);

    Ok(())
}
