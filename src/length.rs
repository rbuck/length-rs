use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::str::FromStr;

use crate::{LengthError, LengthUnit};

/// An absolute physical length with a finite magnitude and a preserved unit.
///
/// Magnitudes may be positive, negative, or zero. Equality compares both
/// magnitude and unit rather than physical equivalence.
///
/// Numerical operations are subject to floating-point rounding.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Length {
    value: f64,
    unit: LengthUnit,
}

impl Length {
    /// Constructs a length with the specified magnitude and unit.
    ///
    /// # Errors
    ///
    /// Returns [`LengthError::NonFiniteValue`] if the magnitude is NaN or infinite.
    pub fn new(value: f64, unit: LengthUnit) -> Result<Self, LengthError> {
        if !value.is_finite() {
            return Err(LengthError::NonFiniteValue);
        }

        Ok(Self { value, unit })
    }

    /// Constructs a zero length in the specified unit.
    pub const fn zero(unit: LengthUnit) -> Self {
        Self { value: 0.0, unit }
    }

    /// Returns the magnitude in the stored unit.
    pub const fn value(self) -> f64 {
        self.value
    }

    /// Returns the stored unit.
    pub const fn unit(self) -> LengthUnit {
        self.unit
    }

    /// Converts this length to the specified unit.
    ///
    /// # Errors
    ///
    /// Returns an error if the conversion overflows or underflows.
    pub fn convert(self, unit: LengthUnit) -> Result<Self, LengthError> {
        let value = self.as_unit(unit)?;

        Ok(Self { value, unit })
    }

    /// Returns the magnitude expressed in the specified unit.
    ///
    /// # Errors
    ///
    /// Returns an error if the conversion overflows or underflows.
    pub fn as_unit(self, unit: LengthUnit) -> Result<f64, LengthError> {
        let value = unit.convert(self.value, self.unit);

        Self::validate_result(value, self.value != 0.0)
    }

    /// Returns whether the magnitude is zero.
    pub fn is_zero(self) -> bool {
        self.value == 0.0
    }

    /// Returns whether the magnitude is greater than zero.
    pub fn is_positive(self) -> bool {
        self.value > 0.0
    }

    /// Returns whether the magnitude is less than zero.
    pub fn is_negative(self) -> bool {
        self.value < 0.0
    }

    /// Returns the absolute length, preserving the unit.
    pub fn abs(self) -> Self {
        Self {
            value: self.value.abs(),
            unit: self.unit,
        }
    }

    /// Adds another length, preserving the left-hand operand's unit.
    ///
    /// # Errors
    ///
    /// Returns an error if conversion or addition exceeds the numerical range.
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
    /// # Errors
    ///
    /// Returns an error if conversion or subtraction exceeds the numerical range.
    pub fn checked_sub(self, rhs: Self) -> Result<Self, LengthError> {
        let rhs = rhs.as_unit(self.unit)?;
        let value = Self::validate_result(self.value - rhs, false)?;

        Ok(Self {
            value,
            unit: self.unit,
        })
    }

    /// Multiplies by a scalar, preserving the unit.
    ///
    /// # Errors
    ///
    /// Returns an error if the scalar is non-finite or the result
    /// overflows or underflows.
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

    /// Divides by a scalar, preserving the unit.
    ///
    /// # Errors
    ///
    /// Returns an error if the divisor is zero or non-finite,
    /// or the result overflows or underflows.
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
    /// # Errors
    ///
    /// Returns an error if the divisor is zero or an intermediate
    /// or final result overflows or underflows.
    pub fn ratio(self, rhs: Self) -> Result<f64, LengthError> {
        if rhs.is_zero() {
            return Err(LengthError::DivisionByZero);
        }

        let rhs = rhs.as_unit(self.unit)?;

        Self::validate_result(self.value / rhs, self.value != 0.0)
    }

    /// Compares physical lengths by converting the right-hand operand
    /// to the left-hand operand's unit.
    ///
    /// Comparison is subject to conversion rounding.
    ///
    /// # Errors
    ///
    /// Returns an error if the conversion overflows or underflows.
    pub fn physical_cmp(self, rhs: Self) -> Result<Ordering, LengthError> {
        let rhs = rhs.as_unit(self.unit)?;

        self.value
            .partial_cmp(&rhs)
            .ok_or(LengthError::NonFiniteValue)
    }

    /// Tests whether the absolute difference is within the specified tolerance.
    ///
    /// Both the other length and tolerance are converted to this length's unit.
    ///
    /// # Errors
    ///
    /// Returns an error if the tolerance is negative or a conversion
    /// overflows or underflows.
    pub fn approx_eq(self, rhs: Self, tolerance: Self) -> Result<bool, LengthError> {
        if tolerance.is_negative() {
            return Err(LengthError::InvalidTolerance);
        }

        let rhs = rhs.as_unit(self.unit)?;
        let tolerance = tolerance.as_unit(self.unit)?;

        Ok((self.value - rhs).abs() <= tolerance)
    }

    /// Rounds to the nearest multiple of the increment, preserving the unit.
    ///
    /// Halfway values are rounded away from zero.
    ///
    /// # Errors
    ///
    /// Returns an error if the increment is not positive or an intermediate
    /// or final result overflows or underflows.
    pub fn round_to(self, increment: Self) -> Result<Self, LengthError> {
        self.round_with(increment, f64::round)
    }

    /// Rounds toward negative infinity to a multiple of the increment,
    /// preserving the unit.
    ///
    /// # Errors
    ///
    /// Returns an error if the increment is not positive or an intermediate
    /// or final result overflows or underflows.
    pub fn floor_to(self, increment: Self) -> Result<Self, LengthError> {
        self.round_with(increment, f64::floor)
    }

    /// Rounds toward positive infinity to a multiple of the increment,
    /// preserving the unit.
    ///
    /// # Errors
    ///
    /// Returns an error if the increment is not positive or an intermediate
    /// or final result overflows or underflows.
    pub fn ceil_to(self, increment: Self) -> Result<Self, LengthError> {
        self.round_with(increment, f64::ceil)
    }

    /// Applies a rounding operation using an increment in the stored unit.
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
impl Add for Length {
    type Output = Result<Self, LengthError>;

    fn add(self, rhs: Self) -> Self::Output {
        self.checked_add(rhs)
    }
}

/// Subtracts two lengths, preserving the left-hand operand's unit.
///
/// Returns the result of [`Length::checked_sub`].
impl Sub for Length {
    type Output = Result<Self, LengthError>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.checked_sub(rhs)
    }
}

/// Multiplies a length by a scalar, preserving the unit.
///
/// Returns the result of [`Length::checked_mul`].
impl Mul<f64> for Length {
    type Output = Result<Self, LengthError>;

    fn mul(self, rhs: f64) -> Self::Output {
        self.checked_mul(rhs)
    }
}

/// Multiplies a scalar by a length, preserving the length's unit.
///
/// Returns the result of [`Length::checked_mul`].
impl Mul<Length> for f64 {
    type Output = Result<Length, LengthError>;

    fn mul(self, rhs: Length) -> Self::Output {
        rhs.checked_mul(self)
    }
}

/// Divides a length by a scalar, preserving the unit.
///
/// Returns the result of [`Length::checked_div`].
impl Div<f64> for Length {
    type Output = Result<Self, LengthError>;

    fn div(self, rhs: f64) -> Self::Output {
        self.checked_div(rhs)
    }
}

/// Divides two lengths to produce a dimensionless ratio.
///
/// Returns the result of [`Length::ratio`].
impl Div<Length> for Length {
    type Output = Result<f64, LengthError>;

    fn div(self, rhs: Length) -> Self::Output {
        self.ratio(rhs)
    }
}

/// Negates the magnitude, preserving the unit.
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
impl fmt::Display for Length {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.value, self.unit)
    }
}

/// Parses a signed decimal magnitude with an optional exponent,
/// followed immediately by a case-sensitive unit symbol.
///
/// # Errors
///
/// Returns an error for invalid syntax, missing or unknown units,
/// numerical overflow, or underflow to zero.
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

/// Returns the byte offset immediately following a valid numerical magnitude.
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
