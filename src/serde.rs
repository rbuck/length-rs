//! String-based serialization and deserialization of lengths and units.

use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;

use ::serde::de::{self, Visitor};
use ::serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{Length, LengthUnit};

/// Serializes a length as its magnitude followed by its unit symbol.
impl Serialize for Length {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

/// Deserializes a length using its string parsing rules.
impl<'de> Deserialize<'de> for Length {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ParseVisitor::<Self>(PhantomData))
    }
}

/// Serializes a unit as its canonical symbol.
impl Serialize for LengthUnit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

/// Deserializes a unit from its case-sensitive symbol.
impl<'de> Deserialize<'de> for LengthUnit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ParseVisitor::<Self>(PhantomData))
    }
}

/// Deserializes string values through their `FromStr` implementation.
struct ParseVisitor<T>(PhantomData<T>);

impl<'de, T> Visitor<'de> for ParseVisitor<T>
where
    T: FromStr,
    T::Err: fmt::Display,
{
    type Value = T;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("a valid physical length or unit symbol")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        value.parse().map_err(E::custom)
    }
}
