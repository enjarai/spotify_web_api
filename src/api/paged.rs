mod all_at_once;
mod lazy;
mod pagination;

pub use all_at_once::*;
pub use lazy::*;
pub use pagination::*;

/// A trait to indicate that an endpoint is pageable.
pub trait Pageable {}

impl<E> Pageable for &E where E: Pageable {}
