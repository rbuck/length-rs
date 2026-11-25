use std::fmt;
use std::str::FromStr;

use crate::LengthError;

/// A supported unit of absolute physical length.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LengthUnit {
    /// Picometer (10⁻¹² meters).
    Picometer,

    /// Nanometer (10⁻⁹ meters).
    Nanometer,

    /// Micrometer (10⁻⁶ meters).
    Micrometer,

    /// Millimeter (10⁻³ meters).
    Millimeter,

    /// Centimeter (10⁻² meters).
    Centimeter,

    /// Decimeter (10⁻¹ meters).
    Decimeter,

    /// Meter (SI base unit of length).
    Meter,

    /// Kilometer (1,000 meters).
    Kilometer,

    /// Mil (1/1,000 of an inch).
    Mil,

    /// International inch (25.4 millimeters).
    Inch,

    /// International foot (12 inches).
    Foot,

    /// International yard (3 feet).
    Yard,

    /// International mile (5,280 feet).
    Mile,

    /// PostScript point (1/72 of an inch).
    Point,

    /// PostScript pica (12 points).
    Pica,

    /// Twip (1/20 of a point).
    Twip,

    /// Astronomical unit (149,597,870,700 meters).
    AstronomicalUnit,

    /// Light-year (distance light travels in a Julian year in vacuum).
    LightYear,

    /// Parsec (648,000/π astronomical units).
    Parsec,
}

impl LengthUnit {
    /// All supported units in declaration order.
    pub const ALL: [Self; 19] = [
        Self::Picometer,
        Self::Nanometer,
        Self::Micrometer,
        Self::Millimeter,
        Self::Centimeter,
        Self::Decimeter,
        Self::Meter,
        Self::Kilometer,
        Self::Mil,
        Self::Inch,
        Self::Foot,
        Self::Yard,
        Self::Mile,
        Self::Point,
        Self::Pica,
        Self::Twip,
        Self::AstronomicalUnit,
        Self::LightYear,
        Self::Parsec,
    ];

    /// Returns the canonical, case-sensitive unit symbol.
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::Picometer => "pm",
            Self::Nanometer => "nm",
            Self::Micrometer => "µm",
            Self::Millimeter => "mm",
            Self::Centimeter => "cm",
            Self::Decimeter => "dm",
            Self::Meter => "m",
            Self::Kilometer => "km",
            Self::Mil => "mil",
            Self::Inch => "in",
            Self::Foot => "ft",
            Self::Yard => "yd",
            Self::Mile => "mi",
            Self::Point => "pt",
            Self::Pica => "pica",
            Self::Twip => "twip",
            Self::AstronomicalUnit => "au",
            Self::LightYear => "ly",
            Self::Parsec => "pc",
        }
    }

    /// Returns the singular, lowercase English name of the unit.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Picometer => "picometer",
            Self::Nanometer => "nanometer",
            Self::Micrometer => "micrometer",
            Self::Millimeter => "millimeter",
            Self::Centimeter => "centimeter",
            Self::Decimeter => "decimeter",
            Self::Meter => "meter",
            Self::Kilometer => "kilometer",
            Self::Mil => "mil",
            Self::Inch => "inch",
            Self::Foot => "foot",
            Self::Yard => "yard",
            Self::Mile => "mile",
            Self::Point => "point",
            Self::Pica => "pica",
            Self::Twip => "twip",
            Self::AstronomicalUnit => "astronomical unit",
            Self::LightYear => "light-year",
            Self::Parsec => "parsec",
        }
    }

    /// Returns the number of meters per unit.
    const fn factor(self) -> f64 {
        match self {
            Self::Picometer => 1e-12,
            Self::Nanometer => 1e-9,
            Self::Micrometer => 1e-6,
            Self::Millimeter => 1e-3,
            Self::Centimeter => 1e-2,
            Self::Decimeter => 1e-1,
            Self::Meter => 1.0,
            Self::Kilometer => 1e3,

            Self::Mil => 0.000_025_4,
            Self::Inch => 0.0254,
            Self::Foot => 0.3048,
            Self::Yard => 0.9144,
            Self::Mile => 1609.344,

            Self::Point => 0.0254 / 72.0,
            Self::Pica => 0.0254 / 6.0,
            Self::Twip => 0.0254 / 1440.0,

            Self::AstronomicalUnit => 149_597_870_700.0,
            Self::LightYear => 9_460_730_472_580_800.0,
            Self::Parsec => 648_000.0 * 149_597_870_700.0 / std::f64::consts::PI,
        }
    }

    /// Converts a magnitude from the source unit to this unit.
    ///
    /// Identical units return the magnitude unchanged. Results follow
    /// IEEE 754 semantics without validation of overflow, underflow,
    /// or non-finite values.
    ///
    /// Use [`crate::Length::convert`] for checked conversion.
    ///
    /// # Examples
    ///
    /// ```
    /// use length::LengthUnit;
    ///
    /// let millimeters = LengthUnit::Millimeter.convert(1.0, LengthUnit::Inch);
    /// assert!((millimeters - 25.4).abs() < 1e-12);
    /// ```
    pub fn convert(self, value: f64, source: Self) -> f64 {
        if self == source {
            return value;
        }

        value * (source.factor() / self.factor())
    }
}

/// Formats the unit as its canonical symbol.
impl fmt::Display for LengthUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.symbol())
    }
}

/// Parses a canonical, case-sensitive unit symbol.
///
/// # Errors
///
/// Returns [`LengthError::UnknownUnit`] if the symbol is not recognized.
impl FromStr for LengthUnit {
    type Err = LengthError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|unit| unit.symbol() == s)
            .ok_or(LengthError::UnknownUnit)
    }
}
