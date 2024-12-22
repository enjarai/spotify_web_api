use serde::{Deserialize, Serialize};

/// List of available genres seed parameter values for [recommendations](https://developer.spotify.com/documentation/web-api/reference/get-recommendations).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Genres {
    pub genres: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn genres() {
        let json = r#"
        {
			"genres": ["alternative", "samba"]
        }
        "#;

        crate::test::assert_deserialized!(Genres, json);
    }
}
