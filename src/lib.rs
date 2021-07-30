mod de;

#[derive(Debug, PartialEq)]
// #[serde(untagged)]
pub enum Possible<T> {
    Some(T),
    None,
    Skip,
}

impl<T> Default for Possible<T> {
    fn default() -> Possible<T> {
        Possible::Skip
    }
}

/// Note that tagged tests are temporary for debugging purposes,
/// the goal is only for untagged results
#[cfg(test)]
mod tagged_tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use serde::Deserialize;

    #[derive(Debug, Deserialize, PartialEq)]
    pub struct Parse {
        test: Possible<i64>,
    }

    #[test]
    fn baseline_with_option() {
        #[derive(Debug, Deserialize, PartialEq)]
        pub struct Parse {
            test: Option<i64>,
        }

        let json = r#"{ "test": 123 }"#;
        let parsed: Parse = serde_json::from_str(json).unwrap();

        assert_eq!(
            parsed,
            Parse { test: Some(123) },
            "Failed to parse expected number value"
        );
    }

    #[test]
    fn with_some_tagged_value() {
        let json = r#"{ "test": { "Some": 123 } }"#;
        let parsed: Parse = serde_json::from_str(json).unwrap();

        assert_eq!(
            parsed,
            Parse {
                test: Possible::Some(123)
            },
            "Failed to parse expected number value"
        );
    }

    #[test]
    fn with_tagged_null_value() {
        let json = r#"{ "test": { "None": null } }"#;
        let parsed: Parse = serde_json::from_str(json).unwrap();

        assert_eq!(
            parsed,
            Parse {
                test: Possible::None,
            },
            "Failed to parse expected null value"
        );
    }

    #[test]
    fn with_tagged_no_field() {
        let json = r#"{ "test": { "Skip": null} }"#;
        let parsed: Parse = serde_json::from_str(json).unwrap();

        assert_eq!(
            parsed,
            Parse {
                test: Possible::Skip,
            },
            "Failed to parse expected field omission"
        );
    }

    #[test]
    fn with_some_untagged_value() {
        let json = r#"{ "test": 123 }"#;
        let parsed: Parse = serde_json::from_str(json).unwrap();

        assert_eq!(
            parsed,
            Parse {
                test: Possible::Some(123),
            },
            "Failed to parse expected number value"
        );
    }

    #[test]
    fn with_untagged_null_value() {
        let json = r#"{ "test": null }"#;
        let parsed: Parse = serde_json::from_str(json).unwrap();

        assert_eq!(
            parsed,
            Parse {
                test: Possible::None,
            },
            "Failed to parse expected null value"
        );
    }

    #[test]
    fn with_untagged_no_field() {
        let json = r#"{ }"#;
        let parsed: Parse = serde_json::from_str(json).unwrap();

        assert_eq!(
            parsed,
            Parse {
                test: Possible::Skip,
            },
            "Failed to parse expected field omission"
        );
    }
}
