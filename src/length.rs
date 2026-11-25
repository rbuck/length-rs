use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::str::FromStr;

use crate::{LengthError, LengthUnit};

/// An absolute physical length with a finite magnitude and a preserved unit.
///
/// A length consists of an `f64` magnitude and a runtime-selected
/// [`LengthUnit`]. Magnitudes may be positive, negative, or zero.
///
/// Unlike representations that normalize measurements to a common base
/// unit, a length retains its unit as part of its value.
///
/// # Construction
///
/// Lengths can be constructed from a numerical magnitude and unit or
/// parsed from a unit-bearing string.
///
/// ```
/// use length::{Length, LengthUnit::*};
///
/// let width = Length::new(2.625, Inch)?;
/// let height: Length = "25.4mm".parse()?;
///
/// assert_eq!(width.to_string(), "2.625in");
/// assert_eq!(height.to_string(), "25.4mm");
/// # Ok::<(), length::LengthError>(())
/// ```
///
/// # Unit conversion
///
/// Use [`Self::convert`] to obtain a length expressed in another unit,
/// or [`Self::as_unit`] to obtain only the converted numerical magnitude.
///
/// ```
/// use length::{Length, LengthUnit::*};
///
/// let width: Length = "2.625in".parse()?;
/// let points = width.convert(Point)?;
///
/// assert_eq!(width.unit(), Inch);
/// assert_eq!(points.unit(), Point);
/// assert!((points.value() - 189.0).abs() < 1e-12);
/// # Ok::<(), length::LengthError>(())
/// ```
///
/// # Arithmetic
///
/// Addition and subtraction accept lengths expressed in different units.
/// The right-hand operand is converted to the unit of the left-hand
/// operand before performing the operation.
///
/// Scalar multiplication and division preserve the unit of the length.
/// Division of one length by another produces a dimensionless ratio.
///
/// | Operation | Result |
/// |-----------|--------|
/// | `Length + Length` | `Result<Length, LengthError>` |
/// | `Length - Length` | `Result<Length, LengthError>` |
/// | `Length * f64` | `Result<Length, LengthError>` |
/// | `f64 * Length` | `Result<Length, LengthError>` |
/// | `Length / f64` | `Result<Length, LengthError>` |
/// | `Length / Length` | `Result<f64, LengthError>` |
/// | `-Length` | `Length` |
///
/// ```
/// use length::{Length, LengthUnit::*};
///
/// let a: Length = "1in".parse()?;
/// let b: Length = "25.4mm".parse()?;
///
/// let sum = (a + b)?;
///
/// assert_eq!(sum.unit(), Inch);
/// assert!((sum.value() - 2.0).abs() < 1e-12);
/// # Ok::<(), length::LengthError>(())
/// ```
///
/// # Equality and comparison
///
/// Equality compares the stored magnitude and unit. Two lengths
/// representing the same physical distance are not necessarily equal.
///
/// Positive and negative zero compare equal when their units match.
///
/// Use [`Self::physical_cmp`] for physical ordering and
/// [`Self::approx_eq`] for comparison within a physical tolerance.
///
/// ```
/// use length::Length;
///
/// let a: Length = "1in".parse()?;
/// let b: Length = "25.4mm".parse()?;
/// let tolerance: Length = "0.000001mm".parse()?;
///
/// assert_ne!(a, b);
/// assert!(a.approx_eq(b, tolerance)?);
/// # Ok::<(), length::LengthError>(())
/// ```
///
/// Physical comparisons are subject to floating-point conversion rounding.
/// They do not provide an exact, unit-independent ordering.
///
/// # Rounding
///
/// Lengths can be rounded to positive physical increments using
/// [`Self::round_to`], [`Self::floor_to`], and [`Self::ceil_to`].
/// These operations preserve the original unit.
///
/// ```
/// use length::Length;
///
/// let width: Length = "2.63in".parse()?;
/// let increment: Length = "0.125in".parse()?;
///
/// assert_eq!(width.round_to(increment)?.to_string(), "2.625in");
/// # Ok::<(), length::LengthError>(())
/// ```
///
/// # Text representation
///
/// The textual representation consists of a numerical magnitude
/// immediately followed by a canonical, case-sensitive unit symbol.
///
/// ```text
/// 2.625in
/// 25.4mm
/// -6pt
/// 0m
/// 10µm
/// 1e-9m
/// ```
///
/// Parsing accepts an optional sign and exponent. Whitespace and
/// noncanonical unit symbols are not accepted.
///
/// Formatting preserves the stored magnitude and unit, but not
/// incidental lexical details such as leading zeros, trailing
/// decimal zeros, or exponent notation.
///
/// # Serialization
///
/// With the optional `serde` feature enabled, lengths implement
/// Serde serialization and deserialization.
///
/// Lengths are serialized as strings using their `Display`
/// representation. Deserialization follows the same validation
/// rules as [`FromStr`].
///
/// Both human-readable and binary serialization formats use the
/// string representation.
///
/// For example, a TOML configuration may contain:
///
/// ```toml
/// width = "2.625in"
/// height = "25.4mm"
/// margin = "9pt"
/// ```
///
/// The original unit is retained without normalization to a common
/// base unit. No separate magnitude or unit fields are required.
///
/// # Numerical behavior
///
/// Magnitudes and conversion factors use native `f64` arithmetic.
/// Ordinary floating-point rounding is expected. Exact decimal
/// arithmetic and exact conversion round trips are not guaranteed.
///
/// Non-finite magnitudes are rejected. Checked numerical operations
/// report overflow and unexpected underflow to zero through
/// [`LengthError`]. Nonzero subnormal values are accepted.
///
/// Intermediate calculations may overflow or underflow even when
/// the mathematical result would otherwise be representable.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Length {
    value: f64,
    unit: LengthUnit,
}

impl Length {
    /// Constructs a length with the specified magnitude and unit.
    ///
    /// The magnitude is stored without conversion or normalization.
    /// Positive, negative, zero, and nonzero subnormal magnitudes
    /// are accepted.
    ///
    /// # Errors
    ///
    /// Returns [`LengthError::NonFiniteValue`] if the magnitude is
    /// NaN or infinite.
    ///
    /// # Examples
    ///
    /// ```
    /// use length::{Length, LengthUnit::*};
    ///
    /// let width = Length::new(2.625, Inch)?;
    /// let offset = Length::new(-6.0, Point)?;
    ///
    /// assert_eq!(width.to_string(), "2.625in");
    /// assert_eq!(offset.to_string(), "-6pt");
    /// # Ok::<(), length::LengthError>(())
    /// ```
    ///
    /// Non-finite magnitudes are rejected:
    ///
    /// ```
    /// use length::{Length, LengthError, LengthUnit::Meter};
    ///
    /// assert_eq!(
    ///     Length::new(f64::NAN, Meter),
    ///     Err(LengthError::NonFiniteValue)
    /// );
    ///
    /// assert_eq!(
    ///     Length::new(f64::INFINITY, Meter),
    ///     Err(LengthError::NonFiniteValue)
    /// );
    /// ```
    pub fn new(value: f64, unit: LengthUnit) -> Result<Self, LengthError> {
        if !value.is_finite() {
            return Err(LengthError::NonFiniteValue);
        }

        Ok(Self { value, unit })
    }

    /// Constructs a zero length in the specified unit.
    ///
    /// The stored magnitude is positive zero. The unit is retained,
    /// allowing zero lengths to participate in unit-preserving arithmetic.
    pub const fn zero(unit: LengthUnit) -> Self {
        Self { value: 0.0, unit }
    }

    /// Returns the magnitude in the stored unit without conversion.
    ///
    /// Use [`Self::as_unit`] to obtain the magnitude expressed in
    /// another unit.
    pub const fn value(self) -> f64 {
        self.value
    }

    /// Returns the stored unit.
    pub const fn unit(self) -> LengthUnit {
        self.unit
    }

    /// Converts this length to the specified unit.
    ///
    /// Returns a new length representing the same physical distance,
    /// subject to floating-point conversion rounding.
    ///
    /// Conversion to the stored unit preserves the original magnitude.
    ///
    /// Use [`Self::as_unit`] when only the converted numerical magnitude
    /// is required.
    ///
    /// # Errors
    ///
    /// Returns [`LengthError::Overflow`] if the converted magnitude
    /// is non-finite, or [`LengthError::Underflow`] if a nonzero
    /// magnitude converts to zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use length::{Length, LengthUnit::*};
    ///
    /// let original: Length = "2.625in".parse()?;
    /// let converted = original.convert(Point)?;
    ///
    /// assert_eq!(original.unit(), Inch);
    /// assert_eq!(converted.unit(), Point);
    /// assert!((converted.value() - 189.0).abs() < 1e-12);
    /// # Ok::<(), length::LengthError>(())
    /// ```
    pub fn convert(self, unit: LengthUnit) -> Result<Self, LengthError> {
        let value = self.as_unit(unit)?;

        Ok(Self { value, unit })
    }

    /// Returns the numerical magnitude expressed in the specified unit.
    ///
    /// Unlike [`Self::convert`], this method returns an `f64` rather
    /// than another length.
    ///
    /// The original magnitude and unit remain unchanged.
    ///
    /// # Errors
    ///
    /// Returns [`LengthError::Overflow`] if the converted magnitude
    /// is non-finite, or [`LengthError::Underflow`] if a nonzero
    /// magnitude converts to zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use length::{Length, LengthUnit::*};
    ///
    /// let width: Length = "2.625in".parse()?;
    /// let millimeters = width.as_unit(Millimeter)?;
    ///
    /// assert!((millimeters - 66.675).abs() < 1e-12);
    /// assert_eq!(width.unit(), Inch);
    /// # Ok::<(), length::LengthError>(())
    /// ```
    pub fn as_unit(self, unit: LengthUnit) -> Result<f64, LengthError> {
        let value = unit.convert(self.value, self.unit);

        Self::validate_result(value, self.value != 0.0)
    }

    /// Returns whether the magnitude is zero.
    ///
    /// Both positive and negative zero are considered zero,
    /// regardless of the stored unit.
    pub fn is_zero(self) -> bool {
        self.value == 0.0
    }

    /// Returns whether the magnitude is strictly greater than zero.
    pub fn is_positive(self) -> bool {
        self.value > 0.0
    }

    /// Returns whether the magnitude is strictly less than zero.
    pub fn is_negative(self) -> bool {
        self.value < 0.0
    }

    /// Returns the absolute value of this length, preserving its unit.
    ///
    /// Negative magnitudes become positive. Negative zero becomes
    /// positive zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use length::Length;
    ///
    /// let offset: Length = "-6pt".parse()?;
    ///
    /// assert!(offset.is_negative());
    /// assert!(offset.abs().is_positive());
    /// assert_eq!(offset.abs().to_string(), "6pt");
    /// # Ok::<(), length::LengthError>(())
    /// ```
    pub fn abs(self) -> Self {
        Self {
            value: self.value.abs(),
            unit: self.unit,
        }
    }

    /// Adds another length, preserving the left-hand operand's unit.
    ///
    /// The right-hand operand is converted to the stored unit before
    /// addition.
    ///
    /// Reversing the operands may produce a different stored unit
    /// and different floating-point rounding.
    ///
    /// The `+` operator provides equivalent behavior.
    ///
    /// # Errors
    ///
    /// Returns [`LengthError::Overflow`] if conversion or addition
    /// produces a non-finite result.
    ///
    /// Returns [`LengthError::Underflow`] if conversion of a nonzero
    /// right-hand operand produces zero.
    ///
    /// A zero sum is permitted.
    ///
    /// # Examples
    ///
    /// ```
    /// use length::{Length, LengthUnit::*};
    ///
    /// let a: Length = "1in".parse()?;
    /// let b: Length = "25.4mm".parse()?;
    ///
    /// let inches = a.checked_add(b)?;
    /// let millimeters = b.checked_add(a)?;
    ///
    /// assert_eq!(inches.unit(), Inch);
    /// assert_eq!(millimeters.unit(), Millimeter);
    ///
    /// assert!((inches.value() - 2.0).abs() < 1e-12);
    /// assert!((millimeters.value() - 50.8).abs() < 1e-12);
    /// # Ok::<(), length::LengthError>(())
    /// ```
    pub fn checked_add(self, rhs: Self) -> Result<Self, LengthError> {
        let rhs = rhs.as_unit(self.unit)?;
        let value = Self::validate_result(self.value + rhs, false)?;

        Ok(Self {
            value,
            unit: self.unit,
        })
    }

    /// Subtracts another length, preserving the left-hand operand's unit.
    ///
    /// The right-hand operand is converted to the stored unit before
    /// subtraction.
    ///
    /// Negative and zero differences are permitted.
    ///
    /// The `-` operator provides equivalent behavior.
    ///
    /// # Errors
    ///
    /// Returns [`LengthError::Overflow`] if conversion or subtraction
    /// produces a non-finite result.
    ///
    /// Returns [`LengthError::Underflow`] if conversion of a nonzero
    /// right-hand operand produces zero.
    ///
    /// A zero difference is permitted.
    ///
    /// # Examples
    ///
    /// ```
    /// use length::{Length, LengthUnit::*};
    ///
    /// let page: Length = "8.5in".parse()?;
    /// let margin: Length = "36pt".parse()?;
    ///
    /// let usable = page.checked_sub(margin)?.checked_sub(margin)?;
    ///
    /// assert_eq!(usable.unit(), Inch);
    /// assert!((usable.value() - 7.5).abs() < 1e-12);
    /// # Ok::<(), length::LengthError>(())
    /// ```
    pub fn checked_sub(self, rhs: Self) -> Result<Self, LengthError> {
        let rhs = rhs.as_unit(self.unit)?;
        let value = Self::validate_result(self.value - rhs, false)?;

        Ok(Self {
            value,
            unit: self.unit,
        })
    }

    /// Multiplies by a dimensionless scalar, preserving the unit.
    ///
    /// Positive, negative, and zero scalars are permitted.
    ///
    /// The `*` operator provides equivalent behavior with the scalar
    /// on either side of the length.
    ///
    /// # Errors
    ///
    /// Returns [`LengthError::NonFiniteValue`] if the scalar is
    /// NaN or infinite.
    ///
    /// Returns [`LengthError::Overflow`] if the product is non-finite,
    /// or [`LengthError::Underflow`] if two nonzero operands produce zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use length::Length;
    ///
    /// let width: Length = "2.625in".parse()?;
    ///
    /// assert_eq!(width.checked_mul(3.0)?.to_string(), "7.875in");
    /// assert_eq!(width.checked_mul(-1.0)?.to_string(), "-2.625in");
    /// assert!(width.checked_mul(0.0)?.is_zero());
    /// # Ok::<(), length::LengthError>(())
    /// ```
    pub fn checked_mul(self, rhs: f64) -> Result<Self, LengthError> {
        if !rhs.is_finite() {
            return Err(LengthError::NonFiniteValue);
        }

        let value = Self::validate_result(self.value * rhs, self.value != 0.0 && rhs != 0.0)?;

        Ok(Self {
            value,
            unit: self.unit,
        })
    }

    /// Divides by a dimensionless scalar, preserving the unit.
    ///
    /// Positive and negative nonzero divisors are permitted.
    ///
    /// The `/` operator provides equivalent behavior when the
    /// right-hand operand is an `f64`.
    ///
    /// Use [`Self::ratio`] to divide by another length.
    ///
    /// # Errors
    ///
    /// Returns [`LengthError::NonFiniteValue`] if the divisor is
    /// NaN or infinite.
    ///
    /// Returns [`LengthError::DivisionByZero`] if the divisor is
    /// positive or negative zero.
    ///
    /// Returns [`LengthError::Overflow`] if the quotient is non-finite,
    /// or [`LengthError::Underflow`] if a nonzero magnitude produces zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use length::{Length, LengthError};
    ///
    /// let width: Length = "8in".parse()?;
    ///
    /// assert_eq!(width.checked_div(4.0)?.to_string(), "2in");
    /// assert_eq!(width.checked_div(0.0), Err(LengthError::DivisionByZero));
    /// # Ok::<(), LengthError>(())
    /// ```
    pub fn checked_div(self, rhs: f64) -> Result<Self, LengthError> {
        if !rhs.is_finite() {
            return Err(LengthError::NonFiniteValue);
        }

        if rhs == 0.0 {
            return Err(LengthError::DivisionByZero);
        }

        let value = Self::validate_result(self.value / rhs, self.value != 0.0)?;

        Ok(Self {
            value,
            unit: self.unit,
        })
    }

    /// Returns the dimensionless ratio of this length to another.
    ///
    /// The right-hand operand is converted to the stored unit before
    /// division. The resulting quotient is an `f64` without a unit.
    ///
    /// The `/` operator provides equivalent behavior when both
    /// operands are lengths.
    ///
    /// # Errors
    ///
    /// Returns [`LengthError::DivisionByZero`] if the divisor is zero.
    ///
    /// Returns [`LengthError::Overflow`] or [`LengthError::Underflow`]
    /// if conversion of the divisor or calculation of the quotient
    /// exceeds the supported numerical range.
    ///
    /// Intermediate conversion may fail even when the mathematical
    /// ratio would otherwise be representable.
    ///
    /// # Examples
    ///
    /// ```
    /// use length::Length;
    ///
    /// let width: Length = "2in".parse()?;
    /// let increment: Length = "12.7mm".parse()?;
    ///
    /// let ratio = width.ratio(increment)?;
    ///
    /// assert!((ratio - 4.0).abs() < 1e-12);
    /// # Ok::<(), length::LengthError>(())
    /// ```
    pub fn ratio(self, rhs: Self) -> Result<f64, LengthError> {
        if rhs.is_zero() {
            return Err(LengthError::DivisionByZero);
        }

        let rhs = rhs.as_unit(self.unit)?;

        Self::validate_result(self.value / rhs, self.value != 0.0)
    }

    /// Compares the physical magnitude of two lengths.
    ///
    /// The right-hand operand is converted to the stored unit before
    /// comparing their numerical magnitudes.
    ///
    /// Unlike ordinary equality, this comparison does not require
    /// matching units.
    ///
    /// # Numerical behavior
    ///
    /// Comparison is subject to floating-point conversion rounding.
    /// Reversing the operands may change the conversion rounding
    /// and therefore the comparison result.
    ///
    /// This method does not guarantee an exact, unit-independent
    /// total ordering and should not be used where strict ordering
    /// guarantees are required.
    ///
    /// Use [`Self::approx_eq`] when comparison within a physical
    /// tolerance is appropriate.
    ///
    /// # Errors
    ///
    /// Returns [`LengthError::Overflow`] or [`LengthError::Underflow`]
    /// if conversion of the right-hand operand fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::cmp::Ordering;
    /// use length::Length;
    ///
    /// let a: Length = "1in".parse()?;
    /// let b: Length = "30mm".parse()?;
    ///
    /// assert_eq!(a.physical_cmp(b)?, Ordering::Less);
    /// assert_eq!(b.physical_cmp(a)?, Ordering::Greater);
    /// # Ok::<(), length::LengthError>(())
    /// ```
    pub fn physical_cmp(self, rhs: Self) -> Result<Ordering, LengthError> {
        let rhs = rhs.as_unit(self.unit)?;

        self.value
            .partial_cmp(&rhs)
            .ok_or(LengthError::NonFiniteValue)
    }

    /// Tests whether the absolute difference between two lengths is
    /// within a specified physical tolerance.
    ///
    /// Both the right-hand operand and the tolerance are converted
    /// to the stored unit before comparison.
    ///
    /// The tolerance is inclusive and must be nonnegative.
    /// A zero tolerance compares the converted numerical magnitudes
    /// without allowing an additional difference.
    ///
    /// This is an absolute tolerance comparison, not a relative
    /// or percentage-based comparison.
    ///
    /// # Numerical behavior
    ///
    /// Comparison is subject to floating-point conversion and
    /// subtraction rounding. Reversing the operands may affect
    /// the result near the tolerance boundary.
    ///
    /// If subtraction overflows, the absolute difference is infinite
    /// and the comparison returns `false`.
    ///
    /// # Errors
    ///
    /// Returns [`LengthError::InvalidTolerance`] if the tolerance
    /// is negative.
    ///
    /// Returns [`LengthError::Overflow`] or [`LengthError::Underflow`]
    /// if conversion of the right-hand operand or tolerance fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use length::Length;
    ///
    /// let nominal: Length = "25.4mm".parse()?;
    /// let measured: Length = "1.001in".parse()?;
    ///
    /// let loose: Length = "0.05mm".parse()?;
    /// let tight: Length = "0.01mm".parse()?;
    ///
    /// assert!(nominal.approx_eq(measured, loose)?);
    /// assert!(!nominal.approx_eq(measured, tight)?);
    /// # Ok::<(), length::LengthError>(())
    /// ```
    pub fn approx_eq(self, rhs: Self, tolerance: Self) -> Result<bool, LengthError> {
        if tolerance.is_negative() {
            return Err(LengthError::InvalidTolerance);
        }

        let rhs = rhs.as_unit(self.unit)?;
        let tolerance = tolerance.as_unit(self.unit)?;

        Ok((self.value - rhs).abs() <= tolerance)
    }

    /// Rounds to the nearest multiple of a positive physical increment,
    /// preserving the stored unit.
    ///
    /// The increment may be expressed in any supported unit.
    /// It is converted to the stored unit before rounding.
    ///
    /// Halfway values of the calculated quotient are rounded away
    /// from zero.
    ///
    /// Conversion and division rounding may affect whether a value
    /// is treated as exactly halfway between increments.
    ///
    /// # Errors
    ///
    /// Returns [`LengthError::InvalidIncrement`] if the increment
    /// is zero or negative.
    ///
    /// Returns [`LengthError::Overflow`] or [`LengthError::Underflow`]
    /// if conversion or an intermediate or final calculation exceeds
    /// the supported numerical range.
    ///
    /// # Examples
    ///
    /// ```
    /// use length::Length;
    ///
    /// let increment: Length = "0.125in".parse()?;
    /// let positive: Length = "2.63in".parse()?;
    /// let negative: Length = "-2.63in".parse()?;
    ///
    /// assert_eq!(positive.round_to(increment)?.to_string(), "2.625in");
    /// assert_eq!(negative.round_to(increment)?.to_string(), "-2.625in");
    /// # Ok::<(), length::LengthError>(())
    /// ```
    ///
    /// Halfway values round away from zero:
    ///
    /// ```
    /// use length::Length;
    ///
    /// let increment: Length = "1mm".parse()?;
    /// let positive: Length = "2.5mm".parse()?;
    /// let negative: Length = "-2.5mm".parse()?;
    ///
    /// assert_eq!(positive.round_to(increment)?.to_string(), "3mm");
    /// assert_eq!(negative.round_to(increment)?.to_string(), "-3mm");
    /// # Ok::<(), length::LengthError>(())
    /// ```
    pub fn round_to(self, increment: Self) -> Result<Self, LengthError> {
        self.round_with(increment, f64::round)
    }

    /// Rounds toward negative infinity to a multiple of a positive
    /// physical increment, preserving the stored unit.
    ///
    /// The increment is converted to the stored unit before rounding.
    ///
    /// Rounding toward negative infinity differs from truncation
    /// toward zero for negative magnitudes.
    ///
    /// # Errors
    ///
    /// Returns [`LengthError::InvalidIncrement`] if the increment
    /// is zero or negative.
    ///
    /// Returns [`LengthError::Overflow`] or [`LengthError::Underflow`]
    /// if conversion or an intermediate or final calculation exceeds
    /// the supported numerical range.
    ///
    /// # Examples
    ///
    /// ```
    /// use length::Length;
    ///
    /// let increment: Length = "1mm".parse()?;
    /// let positive: Length = "2.5mm".parse()?;
    /// let negative: Length = "-2.5mm".parse()?;
    ///
    /// assert_eq!(positive.floor_to(increment)?.to_string(), "2mm");
    /// assert_eq!(negative.floor_to(increment)?.to_string(), "-3mm");
    /// # Ok::<(), length::LengthError>(())
    /// ```
    pub fn floor_to(self, increment: Self) -> Result<Self, LengthError> {
        self.round_with(increment, f64::floor)
    }

    /// Rounds toward positive infinity to a multiple of a positive
    /// physical increment, preserving the stored unit.
    ///
    /// The increment is converted to the stored unit before rounding.
    ///
    /// # Errors
    ///
    /// Returns [`LengthError::InvalidIncrement`] if the increment
    /// is zero or negative.
    ///
    /// Returns [`LengthError::Overflow`] or [`LengthError::Underflow`]
    /// if conversion or an intermediate or final calculation exceeds
    /// the supported numerical range.
    ///
    /// # Examples
    ///
    /// ```
    /// use length::Length;
    ///
    /// let increment: Length = "1mm".parse()?;
    /// let positive: Length = "2.5mm".parse()?;
    /// let negative: Length = "-2.5mm".parse()?;
    ///
    /// assert_eq!(positive.ceil_to(increment)?.to_string(), "3mm");
    /// assert_eq!(negative.ceil_to(increment)?.to_string(), "-2mm");
    /// # Ok::<(), length::LengthError>(())
    /// ```
    pub fn ceil_to(self, increment: Self) -> Result<Self, LengthError> {
        self.round_with(increment, f64::ceil)
    }

    /// Applies a rounding operation to the quotient of the stored
    /// magnitude and a positive increment expressed in the stored unit.
    ///
    /// Validates the intermediate quotient and reconstructed magnitude.
    /// Rounding a nonzero quotient to zero is permitted.
    fn round_with(
        self,
        increment: Self,
        operation: impl FnOnce(f64) -> f64,
    ) -> Result<Self, LengthError> {
        if !increment.is_positive() {
            return Err(LengthError::InvalidIncrement);
        }

        let increment = increment.as_unit(self.unit)?;

        let quotient = Self::validate_result(self.value / increment, self.value != 0.0)?;

        let rounded = operation(quotient);

        let value = Self::validate_result(rounded * increment, rounded != 0.0)?;

        Ok(Self {
            value,
            unit: self.unit,
        })
    }

    /// Rejects non-finite results and unexpected underflow to zero.
    ///
    /// The `expected_nonzero` argument distinguishes unexpected
    /// underflow from operations for which zero is a valid result.
    /// Nonzero subnormal values are accepted.
    fn validate_result(value: f64, expected_nonzero: bool) -> Result<f64, LengthError> {
        if !value.is_finite() {
            return Err(LengthError::Overflow);
        }

        if value == 0.0 && expected_nonzero {
            return Err(LengthError::Underflow);
        }

        Ok(value)
    }
}

/// Adds two lengths, preserving the left-hand operand's unit.
///
/// Returns the result of [`Length::checked_add`].
///
/// # Examples
///
/// ```
/// use length::Length;
///
/// let width: Length = "2.625in".parse()?;
/// let gutter: Length = "0.125in".parse()?;
///
/// assert_eq!((width + gutter)?.to_string(), "2.75in");
/// # Ok::<(), length::LengthError>(())
/// ```
impl Add for Length {
    type Output = Result<Self, LengthError>;

    fn add(self, rhs: Self) -> Self::Output {
        self.checked_add(rhs)
    }
}

/// Subtracts two lengths, preserving the left-hand operand's unit.
///
/// Returns the result of [`Length::checked_sub`].
///
/// # Examples
///
/// ```
/// use length::Length;
///
/// let width: Length = "8.5in".parse()?;
/// let margin: Length = "0.5in".parse()?;
///
/// assert_eq!((width - margin)?.to_string(), "8in");
/// # Ok::<(), length::LengthError>(())
/// ```
impl Sub for Length {
    type Output = Result<Self, LengthError>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.checked_sub(rhs)
    }
}

/// Multiplies a length by a dimensionless scalar, preserving the unit.
///
/// Returns the result of [`Length::checked_mul`].
///
/// # Examples
///
/// ```
/// use length::Length;
///
/// let width: Length = "2.625in".parse()?;
///
/// assert_eq!((width * 3.0)?.to_string(), "7.875in");
/// # Ok::<(), length::LengthError>(())
/// ```
impl Mul<f64> for Length {
    type Output = Result<Self, LengthError>;

    fn mul(self, rhs: f64) -> Self::Output {
        self.checked_mul(rhs)
    }
}

/// Multiplies a dimensionless scalar by a length, preserving the
/// length's unit.
///
/// Returns the result of [`Length::checked_mul`].
///
/// # Examples
///
/// ```
/// use length::Length;
///
/// let width: Length = "2.625in".parse()?;
///
/// assert_eq!((3.0 * width)?.to_string(), "7.875in");
/// # Ok::<(), length::LengthError>(())
/// ```
impl Mul<Length> for f64 {
    type Output = Result<Length, LengthError>;

    fn mul(self, rhs: Length) -> Self::Output {
        rhs.checked_mul(self)
    }
}

/// Divides a length by a dimensionless scalar, preserving the unit.
///
/// Returns the result of [`Length::checked_div`].
///
/// # Examples
///
/// ```
/// use length::Length;
///
/// let width: Length = "8in".parse()?;
///
/// assert_eq!((width / 4.0)?.to_string(), "2in");
/// # Ok::<(), length::LengthError>(())
/// ```
impl Div<f64> for Length {
    type Output = Result<Self, LengthError>;

    fn div(self, rhs: f64) -> Self::Output {
        self.checked_div(rhs)
    }
}

/// Divides two lengths to produce a dimensionless numerical ratio.
///
/// Returns the result of [`Length::ratio`].
///
/// # Examples
///
/// ```
/// use length::Length;
///
/// let width: Length = "8in".parse()?;
/// let increment: Length = "2in".parse()?;
///
/// assert_eq!((width / increment)?, 4.0);
/// # Ok::<(), length::LengthError>(())
/// ```
impl Div<Length> for Length {
    type Output = Result<f64, LengthError>;

    fn div(self, rhs: Length) -> Self::Output {
        self.ratio(rhs)
    }
}

/// Negates the magnitude, preserving the unit.
///
/// Negation is infallible because the negative of a finite `f64`
/// magnitude is also finite.
///
/// # Examples
///
/// ```
/// use length::Length;
///
/// let offset: Length = "6pt".parse()?;
///
/// assert_eq!((-offset).to_string(), "-6pt");
/// assert_eq!(-(-offset), offset);
/// # Ok::<(), length::LengthError>(())
/// ```
impl Neg for Length {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            value: -self.value,
            unit: self.unit,
        }
    }
}

/// Formats the magnitude followed immediately by the canonical unit symbol.
///
/// The magnitude is formatted using the standard `f64` display
/// representation. No whitespace is inserted between the magnitude
/// and unit symbol.
///
/// Formatting is locale-independent and preserves the stored
/// magnitude and unit, but not the original lexical representation.
///
/// The resulting string is accepted by the [`FromStr`] implementation.
///
/// # Examples
///
/// ```
/// use length::Length;
///
/// let width: Length = "+002.6250in".parse()?;
/// let height: Length = "2.54e1mm".parse()?;
///
/// assert_eq!(width.to_string(), "2.625in");
/// assert_eq!(height.to_string(), "25.4mm");
///
/// assert_eq!(width.to_string().parse::<Length>()?, width);
/// # Ok::<(), length::LengthError>(())
/// ```
impl fmt::Display for Length {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.value, self.unit)
    }
}

/// Parses a signed decimal magnitude with an optional exponent,
/// followed immediately by a canonical, case-sensitive unit symbol.
///
/// # Syntax
///
/// The accepted syntax is described by the following ABNF grammar:
///
/// ```text
/// length   = [sign] decimal [exponent] unit
/// sign     = "+" / "-"
/// decimal  = 1*DIGIT ["." 1*DIGIT]
/// exponent = ("e" / "E") [sign] 1*DIGIT
/// ```
///
/// `DIGIT` represents an ASCII decimal digit. The `unit` rule
/// represents any canonical symbol returned by [`LengthUnit::symbol`].
///
/// At least one digit is required before the decimal point.
/// If a decimal point or exponent is present, at least one
/// corresponding digit is required.
///
/// Leading and trailing whitespace, whitespace between the magnitude
/// and unit, digit separators, and noncanonical unit symbols are
/// not accepted.
///
/// NaN and infinity are not valid numerical representations.
///
/// # Errors
///
/// Returns:
///
/// - [`LengthError::InvalidSyntax`] if the numerical syntax is invalid.
/// - [`LengthError::MissingUnit`] if no unit symbol follows the magnitude.
/// - [`LengthError::UnknownUnit`] if the unit symbol is not recognized.
/// - [`LengthError::InvalidNumber`] if floating-point parsing fails.
/// - [`LengthError::Overflow`] if the magnitude parses to a non-finite value.
/// - [`LengthError::Underflow`] if a nonzero magnitude parses to zero.
///
/// Nonzero subnormal magnitudes are accepted.
///
/// # Examples
///
/// Parse decimal and exponential representations:
///
/// ```
/// use length::{Length, LengthUnit::*};
///
/// let width: Length = "2.625in".parse()?;
/// let offset: Length = "-6pt".parse()?;
/// let wavelength: Length = "1e-9m".parse()?;
///
/// assert_eq!(width, Length::new(2.625, Inch)?);
/// assert_eq!(offset, Length::new(-6.0, Point)?);
/// assert_eq!(wavelength, Length::new(1e-9, Meter)?);
/// # Ok::<(), length::LengthError>(())
/// ```
///
/// Invalid representations are rejected:
///
/// ```
/// use length::{Length, LengthError};
///
/// assert_eq!(".5in".parse::<Length>(), Err(LengthError::InvalidSyntax));
/// assert_eq!("1.in".parse::<Length>(), Err(LengthError::InvalidSyntax));
/// assert_eq!("1e+in".parse::<Length>(), Err(LengthError::InvalidSyntax));
/// assert_eq!("25.4".parse::<Length>(), Err(LengthError::MissingUnit));
/// assert_eq!("25.4 mm".parse::<Length>(), Err(LengthError::UnknownUnit));
/// assert_eq!("25.4MM".parse::<Length>(), Err(LengthError::UnknownUnit));
/// ```
///
/// Numerical range violations are rejected:
///
/// ```
/// use length::{Length, LengthError};
///
/// assert_eq!("1e999m".parse::<Length>(), Err(LengthError::Overflow));
/// assert_eq!("1e-999m".parse::<Length>(), Err(LengthError::Underflow));
/// ```
impl FromStr for Length {
    type Err = LengthError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let end = number_end(s)?;
        let (number, symbol) = s.split_at(end);

        if symbol.is_empty() {
            return Err(LengthError::MissingUnit);
        }

        let unit = symbol.parse::<LengthUnit>()?;

        let value = number
            .parse::<f64>()
            .map_err(|_| LengthError::InvalidNumber)?;

        if !value.is_finite() {
            return Err(LengthError::Overflow);
        }

        if value == 0.0 {
            let significand = number.split(['e', 'E']).next().unwrap_or(number);

            if significand.bytes().any(|b| matches!(b, b'1'..=b'9')) {
                return Err(LengthError::Underflow);
            }
        }

        Self::new(value, unit)
    }
}

/// Returns the byte offset immediately following a valid numerical
/// magnitude.
///
/// Recognizes an optional sign, a required decimal integer part,
/// an optional fractional part, and an optional signed exponent.
///
/// Only ASCII characters are consumed, so the returned offset is
/// a valid UTF-8 character boundary even when the following unit
/// symbol contains multibyte characters.
///
/// Numerical range validation and unit parsing are performed separately.
fn number_end(s: &str) -> Result<usize, LengthError> {
    let bytes = s.as_bytes();
    let mut i = 0;

    if matches!(bytes.get(i), Some(b'+' | b'-')) {
        i += 1;
    }

    let start = i;

    while bytes.get(i).is_some_and(u8::is_ascii_digit) {
        i += 1;
    }

    if i == start {
        return Err(LengthError::InvalidSyntax);
    }

    if bytes.get(i) == Some(&b'.') {
        i += 1;

        let start = i;

        while bytes.get(i).is_some_and(u8::is_ascii_digit) {
            i += 1;
        }

        if i == start {
            return Err(LengthError::InvalidSyntax);
        }
    }

    if matches!(bytes.get(i), Some(b'e' | b'E')) {
        i += 1;

        if matches!(bytes.get(i), Some(b'+' | b'-')) {
            i += 1;
        }

        let start = i;

        while bytes.get(i).is_some_and(u8::is_ascii_digit) {
            i += 1;
        }

        if i == start {
            return Err(LengthError::InvalidSyntax);
        }
    }

    Ok(i)
}
