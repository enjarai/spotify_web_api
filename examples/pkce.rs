use spotify_web_api::{
    api::{artists::GetArtist, Query as _},
    auth::scopes,
    model::Artist,
    Spotify,
};
use std::{
    env,
    io::{self, Write},
};

fn main() -> anyhow::Result<()> {
    let client_id = env::var("SPOTIFY_CLIENT_ID")?;

    let mut spotify = Spotify::with_authorization_code_pkce(
        client_id,
        "http://localhost:8888/callback",
        scopes::user_details(),
    )?;

    let user_auth_url = spotify.user_authorization_url();

    println!("\nUser Authorization URL:\n\n{user_auth_url}");
    println!("\nPlease paste the full URL you were redirected to after authorization:\n");
    io::stdout().flush()?;

    let mut redirect_url = String::new();
    io::stdin().read_line(&mut redirect_url)?;

    let redirect_url = redirect_url.trim();

    spotify.request_token_from_redirect_url(redirect_url)?;

    let artist: Artist = GetArtist::from("0559tR6WyukLWH68JIGBuC").query(&spotify)?;

    println!("\n{} -> {}\n", artist.name, artist.external_urls.spotify);

    Ok(())
}
