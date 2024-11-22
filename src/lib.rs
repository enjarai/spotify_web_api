#[cfg(feature = "client_api")]
mod spotify;

#[cfg(feature = "client_api")]
pub mod model;

#[cfg(feature = "client_api")]
pub mod api;

#[cfg(feature = "client_api")]
mod auth;

#[cfg(feature = "client_api")]
pub use auth::{AuthError, AuthResult};

#[cfg(feature = "client_api")]
pub use spotify::{RestError, Spotify, SpotifyError, SpotifyResult};

#[cfg(test)]
mod test;
