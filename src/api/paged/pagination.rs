pub(crate) const MAX_LIMIT: usize = 50;

/// Pagination options for Spotify.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Pagination {
    #[default]
    All,
    Limit(usize),
    Page {
        limit: usize,
        offset: usize,
    },
}

impl Pagination {
    pub(crate) fn limit(self) -> usize {
        match self {
            Self::All => MAX_LIMIT,
            Self::Limit(limit) | Self::Page { limit, .. } => limit.min(MAX_LIMIT),
        }
    }

    pub(crate) fn is_last_page(self, last_page_size: usize, num_results: usize) -> bool {
        match self {
            Self::All => last_page_size < MAX_LIMIT,
            Self::Limit(limit) => limit <= num_results || limit > MAX_LIMIT,
            Self::Page { limit, offset } => last_page_size < limit || offset + limit >= num_results,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pagination_default() {
        assert_eq!(Pagination::default(), Pagination::All);
    }

    #[test]
    fn test_pagination_page_limit() {
        assert_eq!(Pagination::All.limit(), MAX_LIMIT);
        assert_eq!(Pagination::Limit(MAX_LIMIT).limit(), MAX_LIMIT,);
        assert_eq!(Pagination::Limit(1).limit(), 1);
    }
}
