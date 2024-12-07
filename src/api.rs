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
    pub use super::Pageable;
    pub use crate::model::Market;
    pub use derive_builder::Builder;
    pub use endpoint_derive::Endpoint;
}

// pub mod foo {
//     use crate::model::Market;
//     use endpoint_derive::Endpoint;

//     #[derive(Endpoint)]
//     #[endpoint(method = GET, path = "/test/{a}")]
//     pub struct TestEndpoint {
//         a: String,

//         b: Option<Market>,

//         c: Vec<String>,

//         #[endpoint(body)]
//         d: String,

//         #[endpoint(body)]
//         e: String,

//         #[endpoint(body)]
//         f: Option<String>,
//     }
// }
