#[cfg(feature = "client_api")]
mod spotify;

#[cfg(feature = "client_api")]
pub mod model;

#[cfg(feature = "client_api")]
pub mod api;

#[cfg(feature = "client_api")]
pub mod auth;

#[cfg(feature = "client_api")]
pub use spotify::*;

#[cfg(test)]
mod test;
