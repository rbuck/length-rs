# Design

## 1. Purpose

The `length` library provides a strongly typed representation of absolute physical lengths, preserving both the numerical magnitude and the specified unit of measurement.

The library prioritizes simplicity, numerical correctness, interoperability, and predictable behavior across physical scales.

## 2. Design principles

- Preserve the specified unit of measurement.
- Support runtime unit selection.
- Use native Rust numerical arithmetic.
- Maintain a small, explicit public API.
- Provide consistent parsing, formatting, and serialization.
- Require no mandatory external dependencies.
- Forbid unsafe code.

## 3. Scope

The library supports:

- Physical length representation.
- Unit conversion.
- Parsing and formatting.
- Arithmetic and rounding.
- Physical comparison.
- Optional Serde serialization.

The following are explicitly outside the scope:

- Relative or context-dependent lengths.
- General-purpose dimensional analysis.
- Derived quantities such as area and volume.
- Custom unit definitions and registries.
- Custom numerical arithmetic engines.
- Rendering and layout.

## 4. Data model

### 4.1 Length

A length consists of a numerical magnitude and a unit of measurement.

```rust
pub struct Length {
    value: f64,
    unit: LengthUnit,
}
```

The fields are private to maintain numerical invariants.

A length may be positive, negative, or zero. Negative lengths are useful for offsets, displacements, and arithmetic.

NaN and infinite magnitudes are not valid lengths.

Construction and operations that could violate these invariants must return an error.

### 4.2 LengthUnit

Units are represented by a closed enumeration. Units are runtime values rather than generic type parameters.

Each unit defines:

- A unique canonical symbol.
- A human-readable name.
- A conversion factor relative to the meter.

Unit definitions are independent of the numerical magnitude.

### 4.3 LengthError

Errors are represented by a strongly typed, non-exhaustive enumeration implementing the standard Rust `Error` and `Display` traits.

Errors distinguish invalid syntax, invalid numerical magnitudes, missing or unknown units, non-finite values, overflow, underflow, division by zero, and invalid arithmetic arguments.

## 5. Supported units

The meter is the reference unit for all conversions.

| Unit | Symbol | Length in meters |
|---|---|---:|
| Picometer | `pm` | 10⁻¹² |
| Nanometer | `nm` | 10⁻⁹ |
| Micrometer | `µm` | 10⁻⁶ |
| Millimeter | `mm` | 10⁻³ |
| Centimeter | `cm` | 10⁻² |
| Decimeter | `dm` | 10⁻¹ |
| Meter | `m` | 1 |
| Kilometer | `km` | 10³ |
| Mil | `mil` | 0.0000254 |
| Inch | `in` | 0.0254 |
| Foot | `ft` | 0.3048 |
| Yard | `yd` | 0.9144 |
| Mile | `mi` | 1609.344 |
| Point | `pt` | 0.0254 / 72 |
| Pica | `pica` | 0.0254 / 6 |
| Twip | `twip` | 0.0254 / 1440 |
| Astronomical unit | `au` | 149597870700 |
| Light-year | `ly` | 9460730472580800 |
| Parsec | `pc` | 648000 × au / π |

Customary units are based on the international inch, defined as exactly 0.0254 meters.

Typographic units use the PostScript convention of 72 points per inch.

The light-year is based on the Julian year and the defined speed of light in vacuum.

The canonical symbol `pica` avoids ambiguity with the astronomical symbol `pc` for parsec.

## 6. Conversion

Conversion factors are represented using `f64` and express the number of meters per unit.

Conversion from a source unit to a target unit is defined as:

    result = value × (source_factor / target_factor)

The conversion ratio is calculated before multiplication to reduce unnecessary intermediate overflow and underflow.

Conversion between identical units returns the original magnitude without performing arithmetic.

Two levels of conversion are provided:

- `LengthUnit::convert` performs unchecked numerical conversion
  using IEEE 754 floating-point semantics.
- `Length::convert` and `Length::as_unit` perform checked conversion
  and return errors for overflow or underflow to zero.

Converting a length produces a new length without modifying the original.

No implicit normalization to the reference unit occurs during construction, parsing, or serialization.

## 7. Numerical semantics

Magnitudes and conversion factors use IEEE 754 binary64 floating-point representation.

Unit definitions are mathematically exact where applicable. Their floating-point representations and arithmetic results may be approximate.

The library does not guarantee exact decimal arithmetic, correctly rounded conversions, or exact round-trip conversions between different units.

Checked numerical operations follow these rules:

- NaN and infinite input magnitudes are rejected.
- Non-finite numerical results are reported as overflow.
- Nonzero values that round to zero during parsing, conversion,
  multiplication, division, or intermediate rounding calculations
  are reported as underflow.
- Finite subnormal values are permitted.
- Division by zero is rejected.
- Ordinary floating-point rounding is permitted.
- Positive and negative zero are valid magnitudes.

Intermediate calculations may overflow or underflow even when the mathematical final result is representable.

Rounding to an increment requires a strictly positive increment. Halfway values are rounded away from zero. Floor and ceiling operations round toward negative and positive infinity, respectively.

Conversion accuracy must be established through independent reference-based testing.

The internal conversion implementation is not part of the public API.

## 8. Text representation

A length is represented by a decimal magnitude with an optional sign and exponent, immediately followed by a unit symbol.

Examples:

```
2.625in
25.4mm
-6pt
0m
1e-9m
1.5pc
```

The representation is case-sensitive and locale-independent.

Scientific notation is supported to accommodate very small and very large magnitudes.

### 8.1 Grammar

The following grammar uses ABNF as defined by RFC 5234 and case-sensitive string literals as defined by RFC 7405.

```abnf
length   = [sign] decimal [exponent] unit

sign     = "+" / "-"
decimal  = 1*DIGIT ["." 1*DIGIT]
exponent = ("e" / "E") [sign] 1*DIGIT

unit     = %s"pm"
         / %s"nm"
         / %xC2.B5.6D
         / %s"mm"
         / %s"cm"
         / %s"dm"
         / %s"m"
         / %s"km"
         / %s"mil"
         / %s"in"
         / %s"ft"
         / %s"yd"
         / %s"mi"
         / %s"pt"
         / %s"pica"
         / %s"twip"
         / %s"au"
         / %s"ly"
         / %s"pc"
```

The micrometer symbol uses the Unicode MICRO SIGN (U+00B5), encoded as UTF-8. The Greek lowercase letter mu (U+03BC) is not accepted as an alternative.

Whitespace, thousands separators, localized decimal separators, and omitted units are not permitted.

### 8.2 Parsing

`Length` and `LengthUnit` implement `FromStr`.

Parsing validates both syntax and numerical representability.

### 8.3 Formatting

`Length` and `LengthUnit` implement `Display`.

Formatting uses the canonical unit symbol and a decimal magnitude sufficient to recover the stored numerical value when parsed again.

Formatting preserves the unit but does not preserve the original lexical representation.

For example, `2.500in` may be formatted as `2.5in`.

## 9. Arithmetic

The library supports:

- Addition and subtraction of lengths.
- Multiplication and division by scalars.
- Division of one length by another to produce a dimensionless ratio.
- Negation and absolute value.
- Rounding to a specified physical increment.

Arithmetic that can fail returns explicit errors rather than panicking.

Multiplication of two lengths is not supported because it produces a derived physical quantity outside the scope of the library.

### 9.1 Arithmetic operators

Supported operators delegate to their corresponding methods.

| Operator | Equivalent method  | Return type                  |
| -------- | ------------------ | ---------------------------- |
| `a + b`  | `a.checked_add(b)` | `Result<Length, LengthError>` |
| `a - b`  | `a.checked_sub(b)` | `Result<Length, LengthError>` |
| `-a`     | Unary negation     | `Length`                     |

Scalar multiplication, scalar division, and dimensionless ratios are available through `checked_mul`, `checked_div`, and `ratio`, respectively.

### 9.2 Unit propagation

Arithmetic operations that produce a length preserve the unit of the length operand.

For addition and subtraction, the right-hand operand is converted to the unit of the left-hand operand before performing the operation.

| Mathematical expression | Result |
| ----------------------- | ------ |
| `1in + 25.4mm`          | `2in` |
| `25.4mm + 1in`          | `50.8mm` |
| `2in - 25.4mm`          | `1in` |
| `1in × 2`               | `2in` |
| `1in ÷ 2`               | `0.5in` |
| `1in ÷ 25.4mm`          | `1` (dimensionless) |

These examples illustrate mathematical results rather than Rust syntax and are subject to floating-point rounding.

No arithmetic operation implicitly selects a preferred unit or normalizes its result to a reference unit.

Although addition is mathematically commutative, reversing the operands may produce structurally different results because the resulting unit changes.

## 10. Equality and comparison

Structural equality compares both the numerical magnitude and the unit using their native equality semantics.

Consequently, `1in` and `25.4mm` are structurally distinct, even though they represent the same physical length.

Positive and negative zero compare equal when their units match.

Physical ordering is provided by `physical_cmp`. Approximate physical equivalence is provided by `approx_eq`.

Approximate equivalence uses an absolute length tolerance. The tolerance must be nonnegative.

Physical comparisons convert the right-hand operand and, where applicable, the tolerance to the unit of the left-hand operand.

Comparisons are subject to floating-point conversion rounding. Reversing the operands may produce different comparison results.

`PartialOrd` is not implemented because physical ordering would be inconsistent with structural equality.

## 11. Serialization

Serde support is provided through an optional Cargo feature named `serde`.

Both `Length` and `LengthUnit` are serialized as strings:

- `Length` uses its canonical text representation.
- `LengthUnit` uses its canonical unit symbol.

For example:

    width = "2.625in"
    height = "25.4mm"
    margin = "6pt"
    preferred_unit = "mm"

Serialization delegates to `Display`. Deserialization delegates to `FromStr`, ensuring consistent formatting and validation.

Serialization preserves the stored magnitude and unit but does not preserve the original lexical representation.

The serialized representation is independent of the internal conversion implementation.

## 12. Implementation structure

    src/
    ├── lib.rs
    ├── length.rs
    ├── unit.rs
    ├── error.rs
    └── serde.rs

| Module   | Responsibility                                    |
| -------- | ------------------------------------------------- |
| `lib`    | Public API and crate documentation                |
| `length` | Physical length representation and operations     |
| `unit`   | Unit definitions, conversion, parsing, formatting |
| `error`  | Error definitions                                 |
| `serde`  | Optional serialization and deserialization        |

Implementation modules remain private. Public types are exported from the crate root.

The `serde` module is compiled only when the corresponding Cargo feature is enabled.

## 13. Dependencies

The library has no mandatory external dependencies.

Serde is the only optional runtime dependency.

Development dependencies may be used for testing and verification without introducing downstream runtime dependencies.

## 14. Verification

Tests must cover:

- Every ordered pair of supported units.
- Identity conversions.
- Authoritative reference conversions.
- Conversion round trips.
- Unit preservation.
- Parsing and formatting round trips.
- Invalid input.
- Arithmetic and rounding.
- Structural equality and physical comparison.
- Numerical overflow and underflow.
- Floating-point boundary conditions.
- Optional serialization.
- Arithmetic operator equivalence with checked methods.
- Arithmetic operator error propagation.
- Preservation of signed zero during identity conversions.
- Acceptance of finite subnormal magnitudes.

Conversion accuracy must be evaluated against independently calculated, higher-precision reference values.

Tests must distinguish exact numerical expectations from approximate numerical expectations.

## 15. Compatibility

The library uses Rust edition 2024 with a minimum supported Rust version of 1.89.

Platform-specific implementations are not required.

Public API and serialized representation changes follow semantic versioning.

## 16. Downstream usage

A consumer enables Serde support explicitly:

```toml
[dependencies]
length = { version = "0.1", features = ["serde"] }
serde = { version = "1", features = ["derive"] }
```

Their domain model can then directly incorporate physical lengths:

```rust
use length_rs::{Length, LengthUnit};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Dimensions {
    pub width: Length,
    pub height: Length,
    pub margin: Length,
    pub preferred_unit: LengthUnit,
}
```

This is precisely where length-rs provides value. Physical dimensions become first-class domain types rather than strings that every consuming application must independently parse and validate.
