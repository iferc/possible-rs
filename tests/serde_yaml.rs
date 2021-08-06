mod with_possible {
    use possible::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
    pub struct ParseTest {
        // serde(default) is required to use Possible::Void when field is absent
        // this is due to serde_yaml by default assuming missing values should
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
            let serialized = serde_yaml::to_string(&data).unwrap();

            assert_eq!(
                serialized, "---\ntest: 123\n",
                "Failed to parse expected number value"
            );
        }

        #[test]
        fn with_null_value() {
            let data = ParseTest {
                test: Possible::None,
            };
            let serialized = serde_yaml::to_string(&data).unwrap();

            assert_eq!(
                serialized, "---\ntest: ~\n",
                "Failed to parse expected null value"
            );
        }

        #[test]
        fn with_no_field() {
            let data = ParseTest {
                test: Possible::Void,
            };
            let serialized = serde_yaml::to_string(&data).unwrap();

            assert_eq!(
                serialized, "---\n{}\n",
                "Failed to parse expected field omission"
            );
        }
    }

    mod deserialization {
        use super::{ParseTest, Possible};
        use pretty_assertions::assert_eq;

        #[test]
        fn with_some_value() {
            let yaml = r#"test: 123"#;
            let parsed: ParseTest = serde_yaml::from_str(yaml).unwrap();

            assert_eq!(
                parsed,
                ParseTest {
                    test: Possible::Some(123),
                },
                "Failed to parse expected number value"
            );
        }

        #[test]
        fn with_explicit_null_value() {
            let yaml = r#"test: null"#;
            let parsed: ParseTest = serde_yaml::from_str(yaml).unwrap();

            assert_eq!(
                parsed,
                ParseTest {
                    test: Possible::None,
                },
                "Failed to parse expected null value"
            );
        }

        #[test]
        fn with_shorthand_null_value() {
            //todo name
            let yaml = r#"test: ~"#;
            let parsed: ParseTest = serde_yaml::from_str(yaml).unwrap();

            assert_eq!(
                parsed,
                ParseTest {
                    test: Possible::None,
                },
                "Failed to parse expected null value"
            );
        }

        #[test]
        fn with_implicit_null_value() {
            //todo name
            let yaml = r#"test:"#;
            let parsed: ParseTest = serde_yaml::from_str(yaml).unwrap();

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
            let yaml = r#"{}"#;
            let parsed: ParseTest = serde_yaml::from_str(yaml).unwrap();

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
            let serialized = serde_yaml::to_string(&data).unwrap();

            assert_eq!(
                serialized, "---\ntest: 123\n",
                "Failed to parse expected number value"
            );
        }

        #[test]
        fn with_option_none() {
            let data = ParseTest { test: None };
            let serialized = serde_yaml::to_string(&data).unwrap();

            assert_eq!(
                serialized, "---\ntest: ~\n",
                "Failed to parse expected null value"
            );
        }
    }

    mod deserialization {
        use super::ParseTest;
        use pretty_assertions::assert_eq;

        #[test]
        fn with_option_some() {
            let yaml = r#"test: 123"#;
            let parsed: ParseTest = serde_yaml::from_str(yaml).unwrap();

            assert_eq!(
                parsed,
                ParseTest { test: Some(123) },
                "Failed to parse expected number value"
            );
        }

        #[test]
        fn with_explicit_option_none() {
            let yaml = r#"test: null"#;
            let parsed: ParseTest = serde_yaml::from_str(yaml).unwrap();

            assert_eq!(
                parsed,
                ParseTest { test: None },
                "Failed to parse expected null value"
            );
        }

        #[test]
        fn with_shorthand_option_none() {
            let yaml = r#"test: ~"#;
            let parsed: ParseTest = serde_yaml::from_str(yaml).unwrap();

            assert_eq!(
                parsed,
                ParseTest { test: None },
                "Failed to parse expected null value"
            );
        }

        #[test]
        fn with_implicit_option_none() {
            //todo name
            let yaml = r#"test:"#;
            let parsed: ParseTest = serde_yaml::from_str(yaml).unwrap();

            assert_eq!(
                parsed,
                ParseTest { test: None },
                "Failed to parse expected null value"
            );
        }

        #[test]
        fn with_option_missing() {
            let yaml = r#"{}"#;
            let parsed: ParseTest = serde_yaml::from_str(yaml).unwrap();

            assert_eq!(
                parsed,
                ParseTest { test: None },
                "Failed to parse expected missing field"
            );
        }
    }
}
