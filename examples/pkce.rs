use spotify_web_api::{
    api::{endpoints::ArtistEndpoint, Query as _},
    auth::scopes,
    model::Artist,
    Spotify,
};
use std::{
    env,
    io::{self, Write},
};

fn main() {
    let client_id = env::var("SPOTIFY_CLIENT_ID").expect("SPOTIFY_CLIENT_ID must be set");

    let mut spotify = Spotify::with_authorization_code_pkce(
        client_id,
        "http://localhost:8888/callback",
        scopes::user_details(),
    )
    .expect("failed to create client");

    let user_auth_url = spotify.user_authorization_url();

    println!("\nUser Authorization URL:\n\n{user_auth_url}");
    println!("\nPlease paste the full URL you were redirected to after authorization:\n");
    io::stdout().flush().expect("failed to flush stdout");

    let mut redirect_url = String::new();
    io::stdin()
        .read_line(&mut redirect_url)
        .expect("Failed to read input");

    let redirect_url = redirect_url.trim();

    spotify
        .request_token_from_redirect_url(redirect_url)
        .expect("Failed to request token");

    let artist: Artist = ArtistEndpoint::from("0559tR6WyukLWH68JIGBuC")
        .query(&spotify)
        .expect("Failed to get artist info");

    println!("\n{} -> {}\n", artist.name, artist.external_urls.spotify);
}
