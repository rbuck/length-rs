use std::error::Error;
use std::fmt;

/// An error encountered while parsing, converting, or manipulating a length.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum LengthError {
    /// The input does not conform to the length syntax.
    InvalidSyntax,

    /// The magnitude cannot be parsed as a floating-point number.
    InvalidNumber,

    /// The input does not specify a unit.
    MissingUnit,

    /// The unit symbol is not recognized.
    UnknownUnit,

    /// A numerical operand is NaN or infinite.
    NonFiniteValue,

    /// A numerical result exceeds the finite representable range.
    Overflow,

    /// A nonzero numerical result rounds to zero.
    Underflow,

    /// The divisor is zero.
    DivisionByZero,

    /// The rounding increment is zero or negative.
    InvalidIncrement,

    /// The comparison tolerance is negative.
    InvalidTolerance,
}

/// Formats the error as a human-readable message.
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

impl Error for LengthError {}
