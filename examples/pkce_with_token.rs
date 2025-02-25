use spotify_web_api::{
    Spotify,
    api::{Query as _, users::GetCurrentUserProfile},
    auth::scopes,
    model::CurrentUserProfile,
};
use std::{env, fs};

fn main() -> anyhow::Result<()> {
    let client_id = env::var("SPOTIFY_CLIENT_ID")?;
    let token_file_path = env::var("SPOTIFY_TOKEN_FILE_PATH")?;

    let token_string = fs::read_to_string(token_file_path)?;
    let token = serde_json::from_str(&token_string)?;

    let spotify = Spotify::with_authorization_code_pkce(
        client_id,
        "http://localhost:8888/callback",
        scopes::user_details(),
    )?
    .with_token(token);

    let user_profile: CurrentUserProfile = GetCurrentUserProfile.query(&spotify)?;

    println!("{user_profile:#?}");

    Ok(())
}
