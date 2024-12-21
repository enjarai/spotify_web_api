use spotify_web_api::{auth::scopes, Spotify};
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
    )?
    .on_token_refresh(|token| {
        println!("\nToken Refreshed: {token:#?}\n");
    });

    let user_auth_url = spotify.user_authorization_url();

    println!("\nUser Authorization URL:\n\n{user_auth_url}");
    println!("\nPlease paste the full URL you were redirected to after authorization:\n");
    io::stdout().flush()?;

    let mut redirect_url = String::new();
    io::stdin().read_line(&mut redirect_url)?;

    let redirect_url = redirect_url.trim();

    spotify.request_token_from_redirect_url(redirect_url)?;

    spotify.refresh_token()?;

    Ok(())
}
