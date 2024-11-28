mod client;
mod endpoint;
mod error;
mod ignore;
mod params;

pub(crate) mod query;

pub mod albums;
pub mod artists;
pub mod tracks;
pub mod users;

pub use client::{AsyncClient, Client, RestClient};
pub use endpoint::{Endpoint, UrlBase};
pub use error::{ApiError, BodyError};
pub use ignore::{ignore, Ignore};
pub use params::{FormParams, JsonParams, ParamValue, QueryParams};
pub use query::{AsyncQuery, Query};
