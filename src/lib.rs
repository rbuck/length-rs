//! Strongly typed, unit-preserving physical lengths.
//!
//! This crate represents absolute physical lengths while retaining both
//! their magnitude and unit of measurement. Unlike general-purpose
//! dimensional analysis libraries that normalize quantities to base units,
//! measurements retain their units as part of the value.
//!
//! Units are selected at runtime, allowing measurements expressed in
//! different units to share the same [`Length`] type.
//!
//! # Features
//!
//! - Unit-preserving representation and arithmetic.
//! - Runtime-selectable units of measurement.
//! - Metric, customary, typographic, and astronomical units.
//! - Checked conversion and arithmetic.
//! - Structural equality and explicit physical comparison.
//! - Rounding to physical increments.
//! - Parsing and formatting of unit-bearing values.
//! - Optional Serde serialization and deserialization.
//! - Zero mandatory external dependencies.
//!
//! # Getting started
//!
//! A [`Length`] can be constructed from a numerical magnitude and a
//! [`LengthUnit`], or parsed from a unit-bearing string.
//!
//! ```
//! use length::{Length, LengthUnit::*};
//!
//! let width = Length::new(2.625, Inch)?;
//! let height: Length = "25.4mm".parse()?;
//!
//! assert_eq!(width.value(), 2.625);
//! assert_eq!(width.unit(), Inch);
//! assert_eq!(width.to_string(), "2.625in");
//! assert_eq!(height.to_string(), "25.4mm");
//! # Ok::<(), length::LengthError>(())
//! ```
//!
//! # Unit conversion
//!
//! Conversion is explicit and produces a new measurement without
//! modifying the original.
//!
//! ```
//! use length::{Length, LengthUnit::*};
//!
//! let width: Length = "2.625in".parse()?;
//! let points = width.convert(Point)?;
//!
//! assert_eq!(width.unit(), Inch);
//! assert_eq!(points.unit(), Point);
//! assert!((points.value() - 189.0).abs() < 1e-12);
//! # Ok::<(), length::LengthError>(())
//! ```
//!
//! Use [`Length::as_unit`] to obtain a converted numerical magnitude
//! without constructing another length.
//!
//! Numerical values can also be converted directly using
//! [`LengthUnit::convert`].
//!
//! ```
//! use length::LengthUnit::*;
//!
//! let millimeters = Millimeter.convert(1.0, Inch);
//!
//! assert!((millimeters - 25.4).abs() < 1e-12);
//! ```
//!
//! # Runtime unit selection
//!
//! Units are ordinary runtime values rather than generic type parameters.
//! Applications can select units through configuration, user input,
//! or application logic.
//!
//! ```
//! use length::{Length, LengthUnit};
//!
//! let width: Length = "2.625in".parse()?;
//! let preferred_unit: LengthUnit = "mm".parse()?;
//!
//! let converted = width.convert(preferred_unit)?;
//!
//! assert_eq!(converted.unit(), preferred_unit);
//! # Ok::<(), length::LengthError>(())
//! ```
//!
//! # Unit-preserving arithmetic
//!
//! Addition and subtraction accept measurements expressed in different
//! units and preserve the unit of the left-hand operand.
//!
//! ```
//! use length::{Length, LengthUnit::*};
//!
//! let a: Length = "1in".parse()?;
//! let b: Length = "25.4mm".parse()?;
//!
//! let inches = (a + b)?;
//! let millimeters = (b + a)?;
//!
//! assert_eq!(inches.unit(), Inch);
//! assert_eq!(millimeters.unit(), Millimeter);
//!
//! assert!((inches.value() - 2.0).abs() < 1e-12);
//! assert!((millimeters.value() - 50.8).abs() < 1e-12);
//! # Ok::<(), length::LengthError>(())
//! ```
//!
//! Scalar multiplication and division preserve the unit of the length.
//! Dividing one length by another produces a dimensionless ratio.
//!
//! Arithmetic operators that can fail return a `Result`, allowing
//! errors to be handled explicitly.
//!
//! ## Physical layout
//!
//! The following example calculates the total width of a three-column
//! label layout, including the gutters between columns.
//!
//! ```
//! use length::Length;
//!
//! let width: Length = "2.625in".parse()?;
//! let gutter: Length = "0.125in".parse()?;
//! let columns = 3.0;
//!
//! let labels = (width * columns)?;
//! let gutters = (gutter * (columns - 1.0))?;
//! let total = (labels + gutters)?;
//!
//! assert_eq!(total.to_string(), "8.125in");
//! # Ok::<(), length::LengthError>(())
//! ```
//!
//! # Representation and physical equivalence
//!
//! Ordinary equality compares the stored magnitude and unit.
//! Measurements representing the same physical distance may therefore
//! have different representations.
//!
//! Use [`Length::physical_cmp`] for physical ordering or
//! [`Length::approx_eq`] for comparison within a physical tolerance.
//!
//! ```
//! use length::Length;
//!
//! let a: Length = "1in".parse()?;
//! let b: Length = "25.4mm".parse()?;
//! let tolerance: Length = "0.000001mm".parse()?;
//!
//! assert_ne!(a, b);
//! assert!(a.approx_eq(b, tolerance)?);
//! # Ok::<(), length::LengthError>(())
//! ```
//!
//! # Rounding
//!
//! Measurements can be rounded to positive physical increments using
//! [`Length::round_to`], [`Length::floor_to`], and [`Length::ceil_to`].
//! These operations preserve the unit of the original measurement.
//!
//! ```
//! use length::Length;
//!
//! let width: Length = "2.63in".parse()?;
//! let increment: Length = "0.125in".parse()?;
//!
//! let rounded = width.round_to(increment)?;
//!
//! assert_eq!(rounded.to_string(), "2.625in");
//! # Ok::<(), length::LengthError>(())
//! ```
//!
//! # Text representation
//!
//! Measurements use a compact, locale-independent representation
//! consisting of a numerical magnitude immediately followed by a
//! case-sensitive unit symbol.
//!
//! ```text
//! 2.625in
//! 25.4mm
//! -6pt
//! 0m
//! 10µm
//! 1e-9m
//! ```
//!
//! Formatting preserves the stored magnitude and unit, but not incidental
//! lexical details such as leading zeros or exponent notation.
//!
//! See [`LengthUnit`] for supported units and their symbols.
//!
//! # Serialization
//!
//! Enable the optional `serde` feature to serialize and deserialize
//! [`Length`] and [`LengthUnit`] values as strings.
//!
//! This allows measurements expressed in different units to coexist
//! naturally in configuration files and structured data.
//!
//! For example, a TOML configuration may contain:
//!
//! ```toml
//! width = "2.625in"
//! height = "25.4mm"
//! margin = "9pt"
//! preferred_unit = "mm"
//! ```
//!
//! Each measurement retains its unit through serialization and
//! deserialization. No separate unit fields or document-wide
//! measurement system are required.
//!
//! # Numerical behavior
//!
//! Magnitudes and conversion factors use native `f64` arithmetic.
//! Ordinary floating-point rounding is expected, and exact decimal
//! arithmetic and exact conversion round trips are not guaranteed.
//!
//! The checked [`Length`] API rejects non-finite magnitudes and reports
//! arithmetic and conversion errors through [`LengthError`].
//!
//! Intermediate calculations may overflow or underflow even when the
//! mathematical result would otherwise be representable.
//!
//! Direct numerical conversion through [`LengthUnit::convert`] follows
//! native IEEE 754 floating-point behavior.
//!
//! # Scope
//!
//! This crate focuses exclusively on absolute physical lengths.
//! General-purpose dimensional analysis, derived quantities,
//! context-dependent units, custom unit registries, and arbitrary-precision
//! arithmetic are intentionally outside its scope.
//!
//! # Further reading
//!
//! - [Source repository](https://github.com/rbuck/length-rs)
//! - [Design specification](https://github.com/rbuck/length-rs/blob/master/DESIGN.md)
//! - [Runnable examples](https://github.com/rbuck/length-rs/tree/master/examples)
//! - [Contributing guidelines](https://github.com/rbuck/length-rs/blob/master/CONTRIBUTING.md)

#![forbid(unsafe_code)]

mod error;
mod length;
mod unit;

#[cfg(feature = "serde")]
mod serde;

pub use crate::length::Length;
pub use crate::unit::LengthUnit;
pub use error::LengthError;
