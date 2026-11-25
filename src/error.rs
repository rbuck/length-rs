use std::error::Error;
use std::fmt;

/// An error encountered while parsing, converting, or manipulating
/// a physical length.
///
/// Errors distinguish invalid textual representations, invalid numerical
/// operands, numerical range violations, and invalid operation arguments.
///
/// Checked operations on [`crate::Length`] return this error type rather
/// than panicking when an operation cannot be completed.
///
/// Direct numerical conversion through [`crate::LengthUnit::convert`]
/// is unchecked and does not return this error type.
///
/// # Compatibility
///
/// This enumeration is non-exhaustive. Additional error variants may
/// be introduced in future versions. Consumers must include a fallback
/// arm when matching errors.
///
/// # Examples
///
/// Handle an invalid textual representation:
///
/// ```
/// use length::{Length, LengthError};
///
/// assert_eq!("2.625".parse::<Length>(), Err(LengthError::MissingUnit));
/// assert_eq!("2.625xyz".parse::<Length>(), Err(LengthError::UnknownUnit));
/// ```
///
/// Handle an invalid numerical operation:
///
/// ```
/// use length::{Length, LengthError};
///
/// let width: Length = "2.625in".parse()?;
///
/// assert_eq!(width / 0.0, Err(LengthError::DivisionByZero));
/// # Ok::<(), LengthError>(())
/// ```
///
/// Match specific errors while accommodating future variants:
///
/// ```
/// use length::{Length, LengthError};
///
/// match "25.4".parse::<Length>() {
///     Ok(length) => println!("{length}"),
///     Err(LengthError::MissingUnit) => println!("A unit is required"),
///     Err(LengthError::UnknownUnit) => println!("Unsupported unit"),
///     Err(error) => println!("{error}"),
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum LengthError {
    /// The input does not conform to the length syntax.
    ///
    /// Examples include an empty input, a missing numerical magnitude,
    /// or a malformed decimal or exponent.
    InvalidSyntax,

    /// The magnitude cannot be parsed as a floating-point number.
    InvalidNumber,

    /// The input contains a numerical magnitude but does not specify
    /// a unit.
    MissingUnit,

    /// The unit symbol is not recognized.
    ///
    /// Unit symbols must match their canonical, case-sensitive
    /// representations. See [`crate::LengthUnit::symbol`].
    UnknownUnit,

    /// A numerical operand is NaN or infinite.
    ///
    /// Checked length construction and arithmetic require finite
    /// numerical operands.
    NonFiniteValue,

    /// A numerical result exceeds the finite representable range
    /// of `f64`.
    ///
    /// This may occur during parsing, conversion, arithmetic, or
    /// an intermediate calculation.
    Overflow,

    /// A numerical result expected to be nonzero rounds to zero.
    ///
    /// Nonzero subnormal values are accepted. Ordinary floating-point
    /// rounding does not constitute an underflow error.
    Underflow,

    /// The divisor is positive or negative zero.
    ///
    /// Applies to scalar division and ratios between lengths.
    DivisionByZero,

    /// The rounding increment is zero or negative.
    ///
    /// Rounding, flooring, and ceiling require a strictly positive
    /// physical increment.
    InvalidIncrement,

    /// The physical comparison tolerance is negative.
    ///
    /// A zero tolerance is permitted.
    InvalidTolerance,
}

/// Formats the error as a human-readable message.
///
/// Error messages are intended for diagnostics and presentation.
/// Use the error variants rather than their formatted messages
/// when implementing programmatic error handling.
///
/// # Examples
///
/// ```
/// use length::LengthError;
///
/// let error = LengthError::DivisionByZero;
///
/// println!("{error}");
/// ```
impl fmt::Display for LengthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::InvalidSyntax => "invalid length syntax",
            Self::InvalidNumber => "invalid numerical magnitude",
            Self::MissingUnit => "missing length unit",
            Self::UnknownUnit => "unknown length unit",
            Self::NonFiniteValue => "non-finite numerical value",
            Self::Overflow => "numerical overflow",
            Self::Underflow => "numerical underflow",
            Self::DivisionByZero => "division by zero",
            Self::InvalidIncrement => "invalid rounding increment",
            Self::InvalidTolerance => "invalid comparison tolerance",
        })
    }
}

/// Implements the standard Rust error interface.
///
/// Errors are self-contained and do not wrap underlying errors.
///
/// # Examples
///
/// `LengthError` integrates with standard Rust error handling:
///
/// ```
/// use std::error::Error;
/// use length::Length;
///
/// fn main() -> Result<(), Box<dyn Error>> {
///     let width: Length = "2.625in".parse()?;
///     let margin: Length = "9pt".parse()?;
///     let total = (width + margin)?;
///
///     println!("{total}");
///
///     Ok(())
/// }
/// ```
impl Error for LengthError {}
