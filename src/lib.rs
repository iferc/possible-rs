use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Possible<T> {
    Some(T),
    None,
    Skip,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[derive(Debug, Deserialize, PartialEq)]
    pub struct Parse {
        test: Possible<i64>,
    }

    #[test]
    fn with_some_value() -> Result<(), &'static str> {
        let json = r#"{ "test": 123 }"#;
        let parsed: Result<Parse, &str> =
            serde_json::from_str(json).or(Err("Failed to deserialize"));

        assert_eq!(
            parsed,
            Ok(Parse {
                test: Possible::Some(123),
            }),
            "Failed to parse expected number value"
        );

        Ok(())
    }

    #[test]
    fn with_null_value() -> Result<(), &'static str> {
        let json = r#"{ "test": null }"#;
        let parsed: Result<Parse, &str> =
            serde_json::from_str(json).or(Err("Failed to deserialize"));

        assert_eq!(
            parsed,
            Ok(Parse {
                test: Possible::None,
            }),
            "Failed to parse expected null value"
        );

        Ok(())
    }

    #[test]
    fn with_no_field() -> Result<(), &'static str> {
        let json = r#"{ }"#;
        let parsed: Result<Parse, &str> =
            serde_json::from_str(json).or(Err("Failed to deserialize"));

        assert_eq!(
            parsed,
            Ok(Parse {
                test: Possible::Skip,
            }),
            "Failed to parse expected field omission"
        );

        Ok(())
    }
}
