# Possible

Rust library providing a three state enum for differentiating between an explicit null value and the absense of a value. This is intended to be used with `serde` serialization and deserialization to address an ambiguity between null values and the absense of a value where `Option` comflates the two.

`Possible` is an enumerator over three possibilities `Some(T)`, `None`, and `Void`. The intended use is that `Some(T)` is used for storing some data like with an `Option`; `None` represents explicitly null values; and `Void` represents the absense of any value.

`Possible` implements most of the same functionality as `Option`, and can often be used as a direct replacement. Even though there is disambiguation between `None` and `Void`, they will tend to be treated in the same way for most `Option` functionality to behave in an expected way.

## Installation

With [cargo-edit](https://crates.io/crates/cargo-edit), run the following to add this library to your project.

```sh
cargo add possible
```

Otherwise add the following to the `Cargo.toml` file of your project under the `[dependencies]` table section.

```toml
# Under [dependencies]
possible = "0.1.0"
```

## Usage

In it's most simple case, `Possible` can be used in similar ways to `Option`.

```rust
use possible::Possible;

let possible_value: Possible<u32> = Possible::Some(42);
assert_eq!(possible_value, Possible::Some(42));

if let Possible::Some(value) = possible_value {
    assert_eq!(value, 42);
}

let halved_possible_value = possible_value.map(|value| value / 2);
assert_eq!(halved_possible_value, Possible::Some(21));

let unwrapped_value = possible_value.unwrap();
assert_eq!(unwrapped_value, 42);
```

Matching can be done similarly to `Option` type variants.

```rust
use possible::Possible;

let possible_value: Possible<u32> = Default::default(); // defaults to Possible::Void

if let Possible::Void = possible_value {
    // semantically means there is an absense of any value stored
}

match possible_value {
    Possible::Void => { /* the default variant implying no value */ }
    Possible::None => { /* like Option::None, a null or empty value */ }
    Possible::Some(value) => { /* like Option::Some, a value of some type */ }
}

assert_eq!(possible_value.is_void(), true);
assert_eq!(possible_value.is_none(), false);
assert_eq!(possible_value.is_some(), false);
```

The primary use case for this library is paired with `serde` serialization and deserialization from formats like JSON. This is useful when the a null value (or `None`) is a valid value yet the absence of a value is still significant to capture.

```rust
use serde::Serialize;
use serde_json::json;
use possible::Possible;

// serde(skip_serializing_if = "Possible::is_void") is required to implicitly
// omit values that are set as Possible::Void from being serialized

#[derive(Debug, Serialize, PartialEq)]
pub struct OutputJsonData {
    id: i64,

    #[serde(skip_serializing_if = "Possible::is_void")]
    name: Possible<String>,

    #[serde(skip_serializing_if = "Possible::is_void")]
    enabled: Possible<bool>,
}

let intended_output = OutputJsonData {
    id: 1324,
    name: Possible::Void,
    enabled: Possible::Some(true),
};

let serialized = serde_json::to_string(&intended_output).unwrap();
assert_eq!(serialized, r#"{"id":1324,"enabled":true}"#);
```

```rust
use serde::Deserialize;
use possible::Possible;

let input = r#"{
    "id": 1324,
    "name": "Ferris"
}"#;

// serde(default) is required to implicitly parse
// missing values as Possible::Void instead of Possible::None

#[derive(Debug, Deserialize, PartialEq)]
pub struct InputJsonData {
    id: i64,

    #[serde(default)]
    name: Possible<String>,

    #[serde(default)]
    enabled: Possible<bool>,
}

let parsed_input: InputJsonData = serde_json::from_str(input).unwrap();
assert_eq!(parsed_input, InputJsonData {
    id: 1324,
    name: Possible::Some( String::from("Ferris") ),
    enabled: Possible::Void,
});
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)
