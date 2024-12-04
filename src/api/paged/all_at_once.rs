use super::{Pageable, Pagination};
use crate::api::{ApiError, Client, Endpoint, Query};
use serde::de::DeserializeOwned;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Paged<E> {
    pub(crate) endpoint: E,
    pub(crate) pagination: Pagination,
}

/// Collect data from a paged endpoint.
pub fn paged<E>(endpoint: E, pagination: Pagination) -> Paged<E> {
    Paged {
        endpoint,
        pagination,
    }
}

/// Collect all data from a paged endpoint.
pub fn paged_all<E>(endpoint: E) -> Paged<E> {
    paged(endpoint, Pagination::All)
}

/// Collect a limited amount of data from a paged endpoint.
pub fn paged_with_limit<E>(endpoint: E, limit: usize) -> Paged<E> {
    paged(endpoint, Pagination::Limit(limit))
}

impl<E, T, C> Query<Vec<T>, C> for Paged<E>
where
    E: Endpoint + Pageable,
    T: DeserializeOwned + 'static,
    C: Client,
{
    fn query(&self, client: &C) -> Result<Vec<T>, ApiError<C::Error>> {
        self.iter(client).collect()
    }
}
