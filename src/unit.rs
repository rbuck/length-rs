use std::fmt;
use std::str::FromStr;

use crate::LengthError;

/// A supported unit of absolute physical length.
///
/// Units are runtime values that can be selected, compared, parsed,
/// formatted, and used for numerical conversion independently of
/// a [`crate::Length`].
///
/// Each unit has a canonical, case-sensitive [`symbol`](Self::symbol)
/// and a human-readable English [`name`](Self::name).
///
/// # Supported units
///
/// | Domain | Units |
/// |--------|-------|
/// | Metric | `pm`, `nm`, `µm`, `mm`, `cm`, `dm`, `m`, `km` |
/// | Customary | `mil`, `in`, `ft`, `yd`, `mi` |
/// | Typographic | `pt`, `pica`, `twip` |
/// | Astronomical | `au`, `ly`, `pc` |
///
/// Typographic units follow the PostScript convention of 72 points
/// per inch. The symbol `pica` distinguishes the typographic pica
/// from the astronomical parsec (`pc`).
///
/// # Examples
///
/// Select a unit at runtime and use it to construct a physical length:
///
/// ```
/// use length::{Length, LengthUnit};
///
/// let unit: LengthUnit = "in".parse()?;
/// let width = Length::new(2.625, unit)?;
///
/// assert_eq!(unit, LengthUnit::Inch);
/// assert_eq!(width.to_string(), "2.625in");
/// # Ok::<(), length::LengthError>(())
/// ```
///
/// Convert numerical magnitudes without constructing a length:
///
/// ```
/// use length::LengthUnit::*;
///
/// let points = Point.convert(2.625, Inch);
///
/// assert!((points - 189.0).abs() < 1e-12);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LengthUnit {
    /// Picometer (`pm`), 10⁻¹² meters.
    Picometer,

    /// Nanometer (`nm`), 10⁻⁹ meters.
    Nanometer,

    /// Micrometer (`µm`), 10⁻⁶ meters.
    Micrometer,

    /// Millimeter (`mm`), 10⁻³ meters.
    Millimeter,

    /// Centimeter (`cm`), 10⁻² meters.
    Centimeter,

    /// Decimeter (`dm`), 10⁻¹ meters.
    Decimeter,

    /// Meter (`m`), the SI base unit of length.
    Meter,

    /// Kilometer (`km`), 1,000 meters.
    Kilometer,

    /// Mil (`mil`), 1/1,000 of an inch.
    Mil,

    /// International inch (`in`), 25.4 millimeters.
    Inch,

    /// International foot (`ft`), 12 inches.
    Foot,

    /// International yard (`yd`), 3 feet.
    Yard,

    /// International mile (`mi`), 5,280 feet.
    Mile,

    /// PostScript point (`pt`), 1/72 of an inch.
    Point,

    /// PostScript pica (`pica`), 12 points.
    Pica,

    /// Twip (`twip`), 1/20 of a point.
    Twip,

    /// Astronomical unit (`au`), 149,597,870,700 meters.
    AstronomicalUnit,

    /// Light-year (`ly`), the distance light travels in vacuum
    /// during one Julian year of 365.25 days.
    LightYear,

    /// Parsec (`pc`), 648,000/π astronomical units.
    Parsec,
}

impl LengthUnit {
    /// All supported units in declaration order.
    ///
    /// Provides a complete enumeration of the supported units for
    /// applications such as unit selectors, conversion tables,
    /// and measurement catalogs.
    ///
    /// # Examples
    ///
    /// ```
    /// use length::LengthUnit;
    ///
    /// for unit in LengthUnit::ALL {
    ///     println!("{} ({})", unit.name(), unit.symbol());
    /// }
    /// ```
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
    ///
    /// The symbol is used when parsing and formatting units and
    /// physical lengths, and when serializing with the optional
    /// `serde` feature.
    ///
    /// The micrometer symbol uses the Unicode micro sign (`µ`,
    /// U+00B5), not the Greek lowercase letter mu (`μ`, U+03BC).
    ///
    /// # Examples
    ///
    /// ```
    /// use length::LengthUnit::*;
    ///
    /// assert_eq!(Inch.symbol(), "in");
    /// assert_eq!(Millimeter.symbol(), "mm");
    /// assert_eq!(Micrometer.symbol(), "µm");
    /// assert_eq!(Point.symbol(), "pt");
    /// assert_eq!(Pica.symbol(), "pica");
    /// assert_eq!(Parsec.symbol(), "pc");
    /// ```
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
    ///
    /// Names are intended for human-readable presentation.
    /// Use [`Self::symbol`] for parsing, formatting, and data interchange.
    ///
    /// # Examples
    ///
    /// ```
    /// use length::LengthUnit::*;
    ///
    /// assert_eq!(Inch.name(), "inch");
    /// assert_eq!(Foot.name(), "foot");
    /// assert_eq!(AstronomicalUnit.name(), "astronomical unit");
    /// assert_eq!(LightYear.name(), "light-year");
    /// ```
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

    /// Returns the number of meters per unit as an `f64` conversion factor.
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

    /// Converts a numerical magnitude from the source unit to this unit.
    ///
    /// The receiving unit is the destination of the conversion.
    /// The `value` argument is expressed in the `source` unit.
    ///
    /// Conversion uses the ratio of the source and destination
    /// conversion factors:
    ///
    /// ```text
    /// result = value × (source factor / destination factor)
    /// ```
    ///
    /// Identical source and destination units return the magnitude
    /// unchanged, without performing floating-point arithmetic.
    ///
    /// # Numerical behavior
    ///
    /// Results follow native IEEE 754 floating-point semantics.
    /// Overflow, underflow, and non-finite values are not validated.
    /// Ordinary floating-point rounding may occur.
    ///
    /// Use [`crate::Length::convert`] or [`crate::Length::as_unit`]
    /// when checked conversion and explicit error reporting are required.
    ///
    /// # Examples
    ///
    /// Convert between customary and metric units:
    ///
    /// ```
    /// use length::LengthUnit::*;
    ///
    /// let millimeters = Millimeter.convert(1.0, Inch);
    /// let inches = Inch.convert(25.4, Millimeter);
    ///
    /// assert!((millimeters - 25.4).abs() < 1e-12);
    /// assert!((inches - 1.0).abs() < 1e-12);
    /// ```
    ///
    /// Convert between customary and typographic units:
    ///
    /// ```
    /// use length::LengthUnit::*;
    ///
    /// let points = Point.convert(2.625, Inch);
    /// let picas = Pica.convert(72.0, Point);
    /// let inches = Inch.convert(1440.0, Twip);
    ///
    /// assert!((points - 189.0).abs() < 1e-12);
    /// assert!((picas - 6.0).abs() < 1e-12);
    /// assert!((inches - 1.0).abs() < 1e-12);
    /// ```
    ///
    /// Select the destination unit at runtime:
    ///
    /// ```
    /// use length::LengthUnit;
    ///
    /// let destination: LengthUnit = "mm".parse()?;
    /// let value = destination.convert(1.0, LengthUnit::Inch);
    ///
    /// assert!((value - 25.4).abs() < 1e-12);
    /// # Ok::<(), length::LengthError>(())
    /// ```
    ///
    /// Identical units preserve the original magnitude:
    ///
    /// ```
    /// use length::LengthUnit::Meter;
    ///
    /// assert_eq!(Meter.convert(42.0, Meter), 42.0);
    /// assert_eq!(Meter.convert(-0.0, Meter).to_bits(), (-0.0_f64).to_bits());
    /// ```
    pub fn convert(self, value: f64, source: Self) -> f64 {
        if self == source {
            return value;
        }

        value * (source.factor() / self.factor())
    }
}

/// Formats the unit as its canonical symbol.
///
/// The formatted representation is accepted by the [`FromStr`]
/// implementation, allowing units to be converted to and from text.
///
/// # Examples
///
/// ```
/// use length::LengthUnit::*;
///
/// assert_eq!(Inch.to_string(), "in");
/// assert_eq!(Millimeter.to_string(), "mm");
/// assert_eq!(format!("{Point}"), "pt");
/// ```
impl fmt::Display for LengthUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.symbol())
    }
}

/// Parses a canonical, case-sensitive unit symbol.
///
/// Only symbols returned by [`LengthUnit::symbol`] are accepted.
/// Full unit names, aliases, alternative spellings, and surrounding
/// whitespace are not accepted.
///
/// # Errors
///
/// Returns [`LengthError::UnknownUnit`] if the symbol is not recognized.
///
/// # Examples
///
/// Parse units from their canonical symbols:
///
/// ```
/// use length::LengthUnit;
///
/// assert_eq!("in".parse::<LengthUnit>()?, LengthUnit::Inch);
/// assert_eq!("mm".parse::<LengthUnit>()?, LengthUnit::Millimeter);
/// assert_eq!("µm".parse::<LengthUnit>()?, LengthUnit::Micrometer);
/// # Ok::<(), length::LengthError>(())
/// ```
///
/// Noncanonical symbols are rejected:
///
/// ```
/// use length::{LengthError, LengthUnit};
///
/// assert_eq!("IN".parse::<LengthUnit>(), Err(LengthError::UnknownUnit));
/// assert_eq!("inch".parse::<LengthUnit>(), Err(LengthError::UnknownUnit));
/// assert_eq!(" in".parse::<LengthUnit>(), Err(LengthError::UnknownUnit));
/// assert_eq!("um".parse::<LengthUnit>(), Err(LengthError::UnknownUnit));
/// ```
///
/// Every supported unit can be formatted and parsed without loss:
///
/// ```
/// use length::LengthUnit;
///
/// for unit in LengthUnit::ALL {
///     assert_eq!(unit.to_string().parse::<LengthUnit>()?, unit);
/// }
/// # Ok::<(), length::LengthError>(())
/// ```
impl FromStr for LengthUnit {
    type Err = LengthError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|unit| unit.symbol() == s)
            .ok_or(LengthError::UnknownUnit)
    }
}
