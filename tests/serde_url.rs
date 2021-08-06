//! While there is valid test coverage for the [URL query string format](https://url.spec.whatwg.org/#urlencoded-parsing),
//! `Possible` behaves exactly like `Option` due to TOML not supporting an explicit null value
//! type to differentiate between an explicit null and the absense of a value.

mod with_possible {
    use possible::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
    pub struct ParseTest {
        // serde(default) is required to use Possible::Void when field is absent
        // this is due to serde_qs by default assuming missing values should
        // equal a null actual value instead of undefined/void
        #[serde(default)]
        // it seems to not be possible for a type to tell a context struct's serialization
        // to completely omit a field as a default part of Possible
        // the following skip rule is needed to prevent writing the field with a null value
        #[serde(skip_serializing_if = "Possible::is_void")]
        test: Possible<i64>,
    }

    mod serialization {
        use super::{ParseTest, Possible};
        use pretty_assertions::assert_eq;

        #[test]
        fn with_some_value() {
            let data = ParseTest {
                test: Possible::Some(123),
            };
            let serialized = serde_qs::to_string(&data).unwrap();

            assert_eq!(
                serialized, "test=123",
                "Failed to parse expected number value"
            );
        }

        #[test]
        fn with_null_value() {
            let data = ParseTest {
                test: Possible::None,
            };
            let serialized = serde_qs::to_string(&data).unwrap();

            assert_eq!(serialized, "", "Failed to parse expected null value");
        }

        #[test]
        fn with_no_field() {
            let data = ParseTest {
                test: Possible::Void,
            };
            let serialized = serde_qs::to_string(&data).unwrap();

            assert_eq!(serialized, "", "Failed to parse expected field omission");
        }
    }

    mod deserialization {
        use super::{ParseTest, Possible};
        use pretty_assertions::assert_eq;

        #[test]
        fn with_some_value() {
            let url_query = r#"test=123"#;
            let parsed: ParseTest = serde_qs::from_str(url_query).unwrap();

            assert_eq!(
                parsed,
                ParseTest {
                    test: Possible::Some(123),
                },
                "Failed to parse expected number value"
            );
        }

        #[test]
        fn with_null_value() {
            let url_query = r#"test="#;
            let parsed: ParseTest = serde_qs::from_str(url_query).unwrap();

            assert_eq!(
                parsed,
                ParseTest {
                    test: Possible::None,
                },
                "Failed to parse expected null value"
            );
        }

        #[test]
        fn with_no_field() {
            let url_query = r#""#;
            let parsed: ParseTest = serde_qs::from_str(url_query).unwrap();

            assert_eq!(
                parsed,
                ParseTest {
                    test: Possible::Void,
                },
                "Failed to parse expected field omission"
            );
        }
    }
}

mod baseline_with_option {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    pub struct ParseTest {
        test: Option<i64>,
    }

    mod serialization {
        use super::ParseTest;
        use pretty_assertions::assert_eq;

        #[test]
        fn with_option_some() {
            let data = ParseTest { test: Some(123) };
            let serialized = serde_qs::to_string(&data).unwrap();

            assert_eq!(
                serialized, "test=123",
                "Failed to parse expected number value"
            );
        }

        #[test]
        fn with_option_none() {
            let data = ParseTest { test: None };
            let serialized = serde_qs::to_string(&data).unwrap();

            assert_eq!(serialized, "", "Failed to parse expected null value");
        }
    }

    mod deserialization {
        use super::ParseTest;
        use pretty_assertions::assert_eq;

        #[test]
        fn with_option_some() {
            let url_query = r#"test=123"#;
            let parsed: ParseTest = serde_qs::from_str(url_query).unwrap();

            assert_eq!(
                parsed,
                ParseTest { test: Some(123) },
                "Failed to parse expected number value"
            );
        }

        #[test]
        fn with_option_none() {
            let url_query = r#"test="#;
            let parsed: ParseTest = serde_qs::from_str(url_query).unwrap();

            assert_eq!(
                parsed,
                ParseTest { test: None },
                "Failed to parse expected null value"
            );
        }

        #[test]
        fn with_option_missing() {
            let url_query = r#""#;
            let parsed: ParseTest = serde_qs::from_str(url_query).unwrap();

            assert_eq!(
                parsed,
                ParseTest { test: None },
                "Failed to parse expected missing field"
            );
        }
    }
}
