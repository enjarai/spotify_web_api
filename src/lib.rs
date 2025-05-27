//! This library serves as wrapper for the Spotify Web API, providing a convenient way to interact with Spotify's services.
//!
//! Spotify Web API enables the creation of applications that can interact with Spotify's streaming service, such as retrieving content metadata, getting recommendations, creating and managing playlists, or controlling playback.
//!
//! <br>
//!
//! # Examples
//!
//! Client Credentials:
//!
//! ```rust
//! use spotify_web_api::{
//!     api::{artists::GetArtist, Query as _},
//!     model::Artist,
//!     Spotify,
//! };
//!
//! fn main() -> anyhow::Result<()> {
//!     let spotify = Spotify::with_client_credentials("client_id", "client_secret")?;
//!
//!     spotify.request_token()?;
//!
//!     let artist: Artist = GetArtist::from("0559tR6WyukLWH68JIGBuC").query(&spotify)?;
//!
//!     println!("{artist:#?}");
//!
//!     Ok(())
//! }
//! ```
//!
//! Authorization Code with PKCE (async):
//!
//! ```rust
//! use spotify_web_api::{
//!     api::{users::GetCurrentUserProfile, AsyncQuery as _},
//!     auth::scopes,
//!     model::CurrentUserProfile,
//!     Spotify,
//! };
//! use std::io::{self, Write};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let mut spotify = Spotify::with_authorization_code_pkce(
//!         "client_id",
//!         "http://127.0.0.1:8888/callback",
//!         scopes::all(),
//!     )?;
//!
//!     let user_auth_url = spotify.user_authorization_url();
//!
//!     println!("User Authorization URL:\n\n{user_auth_url}");
//!     println!("Please paste the full URL you were redirected to after authorization:\n");
//!     io::stdout().flush()?;
//!
//!     let mut redirect_url = String::new();
//!     io::stdin().read_line(&mut redirect_url)?;
//!     let redirect_url = redirect_url.trim();
//!
//!     spotify.request_token_from_redirect_url(redirect_url).await?;
//!
//!     let user_profile: CurrentUserProfile = GetCurrentUserProfile.query_async(&spotify).await?;
//!
//!     println!("{user_profile:#?}");
//!
//!     Ok(())
//! }
//! ```
//!
//! There are more examples in the [examples](https://github.com/ry-sev/spotify_web_api/tree/main/examples) folder.
//!
//! <br>
//!
//! # Which OAuth flow should I use? ([source](https://developer.spotify.com/documentation/web-api/concepts/authorization))
//!
//! Choosing one flow over the rest depends on the application you are building:
//!
//! - In scenarios where storing the client secret is not safe (e.g. desktop, mobile apps or JavaScript web apps running in the browser), you can use the [authorization code with PKCE](https://developer.spotify.com/documentation/web-api/tutorials/code-pkce-flow), as it provides protection against attacks where the authorization code may be intercepted.
//! - For some applications running on the backend, such as CLIs or daemons, the system authenticates and authorizes the app rather than a user. For these scenarios, [Client credentials](https://developer.spotify.com/documentation/web-api/tutorials/client-credentials-flow) is the typical choice. This flow does not include user authorization, so only endpoints that do not request user information (e.g. user profile data) can be accessed.
//!
//! The following table summarizes the flows' behaviors:
//!
//! | Flow | Access User Resources | Requires Secret Key (Server-Side) | Access Token Refresh |
//! | :--- | :--- | :--- | :--- |
//! | Authorization code with PKCE | Yes | No | Yes |
//! | Client credentials | No | Yes | No |
//!
//! <br>
//!
//! # API
//!
//! Supported endpoints are organized under the [`api`] module. To interact with an endpoint, you can use either the [`api::Query`] or [`api::AsyncQuery`] traits.
//!
//! - [`api::Query`] is designed for blocking code, making it ideal for synchronous workflows or environments where asynchronous execution is unnecessary or not supported. Opt for this when simplicity is key, such as in single-threaded environments or scripts where blocking is acceptable.
//! - [`api::AsyncQuery`] is intended for asynchronous code and integrates seamlessly with an asynchronous runtime of your choice, such as `Tokio` or `async-std`. This approach is particularly useful when working in environments that benefit from non-blocking operations. Use this trait when building applications that require high concurrency or when interacting with other asynchronous code.
//!
//! There are additional helpers to handle different cases:
//! - [`api::ignore`]: Ignore the Spotify response (useful for POST or PUT endpoints).
//! - [`api::paged`]: Fetch results that are paginated.
//! - [`api::raw`]: Return the raw data from Spotify instead of deserializing into a structure.
//!
//! You're not restricted to the predefined endpoints; you can define your own by implementing the [`api::Endpoint`] trait. [See example](https://github.com/ry-sev/spotify_web_api/blob/main/examples/creds_custom_endpoint.rs).
//!
//! All endpoints return data types chosen by the caller, provided these types implement `serde`'s `Deserialize` trait. The library offers predefined structs in the [`model`] module, but you are free to use your own structs by implementing the `Deserialize` trait. This flexibility is particularly useful when a custom data structure better suits the your needs or when avoiding the overhead of deserializing the entire response is desirable. [See example](https://github.com/ry-sev/spotify_web_api/blob/main/examples/creds_custom_model.rs).
//!
//! <br>
//!
//! # Feature Flags
//!
//! A set of [feature flags](https://doc.rust-lang.org/cargo/reference/features.html#the-features-section) are available to customize the data models. **These are enabled by default**, but you can disable them to reduce the size of the compiled library or to avoid unnecessary data in your application.
//!
//! - `markets` - Enables the `available_markets` field in various models, such as [`model::Track`]. This field contains a list of markets where the content is available.
//! - `page_items` - Enables the field in various models that contain paginated items, such as the `tracks` field in [`model::Playlist`].

mod spotify;

pub mod api;
pub mod auth;
pub mod model;
pub use spotify::*;

#[cfg(test)]
mod test;
