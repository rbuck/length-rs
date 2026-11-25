//! Strongly typed, unit-preserving physical lengths.
//!
//! This crate provides physical length representation, unit conversion,
//! arithmetic, comparison, parsing, and formatting.
//!
//! Lengths retain their specified units rather than being normalized
//! to a common reference unit.

#![forbid(unsafe_code)]

mod error;
mod length;
mod unit;

#[cfg(feature = "serde")]
mod serde;

pub use crate::length::Length;
pub use crate::unit::LengthUnit;
pub use error::LengthError;
