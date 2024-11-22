/// Asserts that the given JSON string can be deserialized into the given type.
#[cfg(feature = "client_api")]
macro_rules! assert_deserialized {
    ($type:ty, $json:expr) => {
        match serde_json::from_str::<$type>($json) {
            Ok(_) => {}
            Err(e) => panic!("Failed to deserialize JSON data: {e}"),
        }
    };
}

#[cfg(feature = "client_api")]
pub(crate) use assert_deserialized;

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn a() {
//         let json = r#"

//         "#;

//         crate::test::assert_deserialized!(A, json);
//     }

//     #[test]
//     fn b() {
//         let json = r#"

//         "#;

//         crate::test::assert_deserialized!(B, json);
//     }
// }
