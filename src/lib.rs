mod boolean;
mod copy;
mod default;
mod deref;
mod filter;
mod from;
mod introspection;
mod iter;
mod map;
mod refs;
mod replace;
mod serde;
mod transpose;
mod unwrap;
mod zip;

/// Three state enum for differentiating between an explicit null value and the absense of a value
#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Copy)]
pub enum Possible<T> {
    Some(T),
    None,
    Void,
}
