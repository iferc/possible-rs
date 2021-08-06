//! While there is valid test coverage for the TOML format, `Possible` behaves exactly
//! like `Option` due to TOML not supporting an explicit null value type to differentiate
//! between an explicit null and the absense of a value.

mod with_possible {
    use possible::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
    pub struct ParseTest {
        // serde(default) is required to use Possible::Void when field is absent
        // this is due to toml by default assuming missing values should
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
            let serialized = toml::to_string(&data).unwrap();

            assert_eq!(
                serialized.trim(),
                r#"test = 123"#,
                "Failed to parse expected number value"
            );
        }

        #[test]
        fn with_null_value() {
            let data = ParseTest {
                test: Possible::None,
            };
            let serialized = toml::to_string(&data).unwrap();

            assert_eq!(serialized, r#""#, "Failed to parse expected field omission");
        }

        #[test]
        fn with_no_field() {
            let data = ParseTest {
                test: Possible::Void,
            };
            let serialized = toml::to_string(&data).unwrap();

            assert_eq!(serialized, r#""#, "Failed to parse expected field omission");
        }
    }

    mod deserialization {
        use super::{ParseTest, Possible};
        use pretty_assertions::assert_eq;

        #[test]
        fn with_some_value() {
            let json = r#"test = 123"#;
            let parsed: ParseTest = toml::from_str(json).unwrap();

            assert_eq!(
                parsed,
                ParseTest {
                    test: Possible::Some(123),
                },
                "Failed to parse expected number value"
            );
        }

        #[test]
        fn with_no_field() {
            let json = r#""#;
            let parsed: ParseTest = toml::from_str(json).unwrap();

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
            let serialized = toml::to_string(&data).unwrap();

            assert_eq!(
                serialized.trim(),
                r#"test = 123"#,
                "Failed to parse expected number value"
            );
        }

        #[test]
        fn with_option_none() {
            let data = ParseTest { test: None };
            let serialized = toml::to_string(&data).unwrap();

            assert_eq!(serialized, r#""#, "Failed to parse expected null value");
        }
    }

    mod deserialization {
        use super::ParseTest;
        use pretty_assertions::assert_eq;

        #[test]
        fn with_option_some() {
            let toml = r#"test = 123"#;
            let parsed: ParseTest = toml::from_str(toml).unwrap();

            assert_eq!(
                parsed,
                ParseTest { test: Some(123) },
                "Failed to parse expected number value"
            );
        }

        #[test]
        fn with_option_missing() {
            let toml = r#""#;
            let parsed: ParseTest = toml::from_str(toml).unwrap();

            assert_eq!(
                parsed,
                ParseTest { test: None },
                "Failed to parse expected missing field"
            );
        }
    }
}
