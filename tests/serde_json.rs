mod with_possible {
    use possible::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
    pub struct Parse {
        // serde(default) is required to use Possible::Void when field is absent
        // this is due to serde_json by default assuming missing values should
        // equal a null actual value instead of undefined/void
        #[serde(default)]
        // it seems to not be possible for a type to tell a context struct's serialization
        // to completely omit a field as a default part of Possible
        // the following skip rule is needed to prevent writing the field with a null value
        #[serde(skip_serializing_if = "Possible::is_void")]
        test: Possible<i64>,
    }

    mod serialization {
        use super::{Parse, Possible};
        use pretty_assertions::assert_eq;

        #[test]
        fn with_some_value() {
            let data = Parse {
                test: Possible::Some(123),
            };
            let serialized = serde_json::to_string(&data).unwrap();

            assert_eq!(
                serialized, r#"{"test":123}"#,
                "Failed to parse expected number value"
            );
        }

        #[test]
        fn with_null_value() {
            let data = Parse {
                test: Possible::None,
            };
            let serialized = serde_json::to_string(&data).unwrap();

            assert_eq!(
                serialized, r#"{"test":null}"#,
                "Failed to parse expected null value"
            );
        }

        #[test]
        fn with_no_field() {
            let data = Parse {
                test: Possible::Void,
            };
            let serialized = serde_json::to_string(&data).unwrap();

            assert_eq!(
                serialized, r#"{}"#,
                "Failed to parse expected field omission"
            );
        }
    }

    mod deserialization {
        use super::{Parse, Possible};
        use pretty_assertions::assert_eq;

        #[test]
        fn with_some_value() {
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
        fn with_null_value() {
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
        fn with_no_field() {
            let json = r#"{ }"#;
            let parsed: Parse = serde_json::from_str(json).unwrap();

            assert_eq!(
                parsed,
                Parse {
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
    pub struct Parse {
        test: Option<i64>,
    }

    mod serialization {
        use super::Parse;
        use pretty_assertions::assert_eq;

        #[test]
        fn with_option_some() {
            let data = Parse { test: Some(123) };
            let serialized = serde_json::to_string(&data).unwrap();

            assert_eq!(
                serialized, r#"{"test":123}"#,
                "Failed to parse expected number value"
            );
        }

        #[test]
        fn with_option_none() {
            let data = Parse { test: None };
            let serialized = serde_json::to_string(&data).unwrap();

            assert_eq!(
                serialized, r#"{"test":null}"#,
                "Failed to parse expected null value"
            );
        }
    }

    mod deserialization {
        use super::Parse;
        use pretty_assertions::assert_eq;

        #[test]
        fn with_option_some() {
            let json = r#"{ "test": 123 }"#;
            let parsed: Parse = serde_json::from_str(json).unwrap();

            assert_eq!(
                parsed,
                Parse { test: Some(123) },
                "Failed to parse expected number value"
            );
        }

        #[test]
        fn with_option_none() {
            let json = r#"{ "test": null }"#;
            let parsed: Parse = serde_json::from_str(json).unwrap();

            assert_eq!(
                parsed,
                Parse { test: None },
                "Failed to parse expected null value"
            );
        }

        #[test]
        fn with_option_missing() {
            let json = r#"{ }"#;
            let parsed: Parse = serde_json::from_str(json).unwrap();

            assert_eq!(
                parsed,
                Parse { test: None },
                "Failed to parse expected missing field"
            );
        }
    }
}
