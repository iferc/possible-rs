# Possible

Rust library providing a three state enum for differentiating between an explicit null value and the absense of a value. This is intended to be used with `serde` serialization and deserialization to address an ambiguity between null values and the absense of a value where `Option` comflates the two.

`Possible` is an enumerator over three possibilities `Some(T)`, `None`, and `Void`. The intended use is that `Some(T)` is used for storing some data like with an `Option`; `None` represents explicitly null values; and `Void` represents the absense of any value.

`Possible` implements most of the same functionality as `Option`, and can often be used as a direct replacement. Even though there is disambiguation between `None` and `Void`, they will tend to be treated in the same way for most `Option` functionality to behave in an expected way.

Note that there is some documentation which was pulled from `Option` which might not explicitly mention `Void` cases but none-the-less are covered.

See usage examples in the [tests](./tests/) folder.
