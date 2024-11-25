use spotify_web_api::{
    api::{endpoints::ArtistEndpoint, Query as _},
    model::Artist,
    Spotify,
};
use std::env;

fn main() {
    let client_id = env::var("SPOTIFY_CLIENT_ID").expect("SPOTIFY_CLIENT_ID must be set");
    let client_secret =
        env::var("SPOTIFY_CLIENT_SECRET").expect("SPOTIFY_CLIENT_SECRET must be set");

    let mut spotify = Spotify::with_client_credentials(client_id, client_secret)
        .expect("failed to create client");

    spotify.request_token().expect("Failed to request token");

    let artist: Artist = ArtistEndpoint::from("0559tR6WyukLWH68JIGBuC")
        .query(&spotify)
        .expect("Failed to get artist info");

    println!("\n{} -> {}\n", artist.name, artist.external_urls.spotify);
}
