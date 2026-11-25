//! String-based serialization and deserialization of physical lengths
//! and units.
//!
//! Enabled through the optional `serde` feature.
//!
//! Both [`Length`] and [`LengthUnit`] are represented as strings,
//! independently of whether the serialization format is human-readable
//! or binary.
//!
//! # Representation
//!
//! | Rust type | Serialized representation |
//! |-----------|---------------------------|
//! | `Length` | `"2.625in"` |
//! | `Length` | `"25.4mm"` |
//! | `Length` | `"-6pt"` |
//! | `LengthUnit` | `"in"` |
//! | `LengthUnit` | `"mm"` |
//!
//! Serialization uses the canonical `Display` representation.
//! Deserialization follows the corresponding `FromStr` parsing rules.
//!
//! The magnitude and unit are preserved without normalizing measurements
//! to a common base unit. Incidental lexical details, such as trailing
//! decimal zeros and exponent notation, are not preserved.
//!
//! Invalid representations are reported through the deserializer's
//! error mechanism.

use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;

use ::serde::de::{self, Visitor};
use ::serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{Length, LengthUnit};

/// Serializes a physical length as its magnitude followed by its
/// canonical unit symbol.
///
/// The serialized representation is identical to the `Display`
/// representation of the length. The original unit is retained.
///
/// # Representation
///
/// For example, a length constructed from a magnitude of `2.625`
/// and the unit [`LengthUnit::Inch`] is serialized as `"2.625in"`.
///
/// A length is serialized as a single string rather than a structure
/// containing separate magnitude and unit fields.
///
/// This representation applies to both human-readable and binary
/// serialization formats.
///
/// # Example
///
/// With the `serde` feature enabled, a structure containing physical
/// lengths can be represented in TOML as:
///
/// ```toml
/// width = "2.625in"
/// height = "25.4mm"
/// margin = "9pt"
/// ```
///
/// Measurements expressed in different units can coexist within the
/// same structure without requiring a document-wide measurement unit.
impl Serialize for Length {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

/// Deserializes a physical length from a unit-bearing string.
///
/// The input must conform to the same syntax accepted by the
/// [`Length`] implementation of [`FromStr`].
///
/// The magnitude and unit are retained as represented by the parsed
/// value, without conversion to a common base unit.
///
/// # Errors
///
/// Returns a deserialization error if the input is not a string or
/// does not represent a valid physical length.
///
/// Parsing errors are reported through the deserializer's error
/// mechanism.
///
/// # Examples
///
/// Deserialize a length directly from a Serde string deserializer:
///
/// ```
/// use length::{Length, LengthUnit};
/// use serde::Deserialize;
/// use serde::de::value::{Error, StrDeserializer};
///
/// let input = StrDeserializer::<Error>::new("2.625in");
/// let width = Length::deserialize(input)?;
///
/// assert_eq!(width.value(), 2.625);
/// assert_eq!(width.unit(), LengthUnit::Inch);
/// assert_eq!(width.to_string(), "2.625in");
/// # Ok::<(), Error>(())
/// ```
///
/// Invalid representations are rejected:
///
/// ```
/// use length::{Length, LengthUnit};
/// use serde::Deserialize;
/// use serde::de::value::{Error, StrDeserializer};
///
/// let input = StrDeserializer::<Error>::new("2.625");
///
/// assert!(Length::deserialize(input).is_err());
/// ```
impl<'de> Deserialize<'de> for Length {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ParseVisitor::<Self>(PhantomData))
    }
}

/// Serializes a unit as its canonical, case-sensitive symbol.
///
/// The serialized representation is identical to the string returned
/// by [`LengthUnit::symbol`].
///
/// Unit names, enum variant names, and numeric discriminants are not
/// used as serialized representations.
///
/// # Representation
///
/// | Unit | Serialized representation |
/// |------|---------------------------|
/// | [`LengthUnit::Inch`] | `"in"` |
/// | [`LengthUnit::Millimeter`] | `"mm"` |
/// | [`LengthUnit::Point`] | `"pt"` |
/// | [`LengthUnit::Pica`] | `"pica"` |
/// | [`LengthUnit::Parsec`] | `"pc"` |
///
/// This representation applies to both human-readable and binary
/// serialization formats.
impl Serialize for LengthUnit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

/// Deserializes a unit from its canonical, case-sensitive symbol.
///
/// The input must match a symbol returned by [`LengthUnit::symbol`].
/// Full unit names, aliases, alternative spellings, and surrounding
/// whitespace are not accepted.
///
/// # Errors
///
/// Returns a deserialization error if the input is not a string or
/// contains an unrecognized unit symbol.
///
/// # Examples
///
/// Deserialize a unit directly from a Serde string deserializer:
///
/// ```
/// use length::LengthUnit;
/// use serde::Deserialize;
/// use serde::de::value::{Error, StrDeserializer};
///
/// let input = StrDeserializer::<Error>::new("mm");
/// let unit = LengthUnit::deserialize(input)?;
///
/// assert_eq!(unit, LengthUnit::Millimeter);
/// # Ok::<(), Error>(())
/// ```
///
/// Noncanonical symbols are rejected:
///
/// ```
/// use length::LengthUnit;
/// use serde::Deserialize;
/// use serde::de::value::{Error, StrDeserializer};
///
/// let input = StrDeserializer::<Error>::new("millimeter");
///
/// assert!(LengthUnit::deserialize(input).is_err());
/// ```
impl<'de> Deserialize<'de> for LengthUnit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ParseVisitor::<Self>(PhantomData))
    }
}

/// Deserializes string values through their [`FromStr`] implementation.
///
/// Converts parsing errors into the deserializer's error type,
/// ensuring that deserialization uses the same validation rules
/// as ordinary string parsing.
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
