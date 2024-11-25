mod spotify;

pub mod api;
pub mod auth;
pub mod model;
pub use spotify::*;

#[cfg(test)]
mod test;
