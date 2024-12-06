mod client;
mod endpoint;
mod error;
mod ignore;
mod paged;
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
pub use paged::*;
pub use params::{FormParams, JsonParams, ParamValue, QueryParams};
pub use query::{AsyncQuery, Query};

mod prelude {
    pub use super::{BodyError, Endpoint, JsonParams, Pageable, QueryParams};
    pub use crate::model::Market;
    pub use derive_builder::Builder;
    pub use http::Method;
    pub use serde_json::json;
    pub use std::borrow::Cow;
}
