use super::Possible;
use serde::{de::Visitor, Deserialize, Deserializer};
use serde::{Serialize, Serializer};
use std::{error::Error, fmt, marker::PhantomData};

struct PossibleVisitor<T>(PhantomData<T>);

impl<'de, T> Visitor<'de> for PossibleVisitor<T>
where
    T: Deserialize<'de>,
{
    type Value = Possible<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("possible")
    }

    #[inline]
    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Possible::Void)
    }

    #[inline]
    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Possible::None)
    }

    #[inline]
    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize(deserializer).map(Possible::Some)
    }
}

impl<'de, T> Deserialize<'de> for Possible<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_option(PossibleVisitor(PhantomData))
    }
}

impl<T> Serialize for Possible<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Possible::Some(ref value) => serializer.serialize_some(value),
            Possible::None => serializer.serialize_none(),
            Possible::Void => serializer.serialize_unit(),
        }
    }
}
