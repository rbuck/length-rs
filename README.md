# length-rs

A lightweight Rust library for representing, converting, and manipulating absolute physical lengths while preserving their units of measurement.

**A length is more than a number. Its unit is part of its identity.**

Unlike general-purpose dimensional analysis libraries that typically normalize quantities to base units, `length-rs` retains both the magnitude and the unit as part of the value. Units are selected at runtime rather than through generic type parameters.

This makes `length-rs` particularly useful in applications where measurements must retain their meaning across computation, configuration, serialization, and presentation.

## Why length-rs?

Consider a physical measurement specified as `2.625in`.

In many measurement libraries, this value is converted into an internal base unit, such as meters. Although the physical quantity is preserved, the original unit of measurement may no longer be part of the value.

`length-rs` takes a different approach:

``` rust
use length_rs::{Length, LengthUnit::*};

let width: Length = "2.625in".parse()?;

assert_eq!(width.value(), 2.625);
assert_eq!(width.unit(), Inch);
assert_eq!(width.to_string(), "2.625in");
```

The magnitude and unit remain together until an explicit conversion is requested.

This distinction matters when working with:

- Printing, publishing, and typography.
- Document generation and page layout.
- Engineering and architectural measurements.
- Manufacturing and fabrication.
- Paper sizes, labels, and packaging.
- Configuration files and measurement catalogs.
- Structured data interchange.
- Scientific applications involving physical distances.

An application can accept millimeters, inches, points, or any other supported unit without imposing a single internal representation on its users.

## Design philosophy

`length-rs` occupies a deliberately narrow space between numerical conversion utilities and general-purpose dimensional analysis systems.

| Capability | Scalar conversion utilities | Type-based dimensional systems | General runtime quantity systems | length-rs |
| --- | --- | --- | --- | --- |
| Strongly typed physical lengths | Typically no | Yes | Yes | Yes |
| Unit retained as part of the value | Typically no | Varies | Varies | Yes |
| Runtime unit selection | Varies | Varies | Yes | Yes |
| Mixed-unit arithmetic | Typically no | Yes | Yes | Yes |
| Compact unit-bearing text | Varies | Varies | Varies | Yes |
| General dimensional algebra | No | Yes | Typically yes | No |
| Focused exclusively on length | Varies | No | No | Yes |
| Mandatory external dependencies | Varies | Varies | Varies | None |

General-purpose libraries solve important problems involving dimensional correctness, derived quantities, and relationships between physical dimensions.

`length-rs` solves a different problem: representing physical lengths as self-contained, unit-preserving values with a small, predictable API.

Its defining characteristics are:

- **Unit preservation.** The unit is part of the value, not merely an input or output format.
- **Runtime flexibility.** Every supported unit uses the same `Length` type.
- **Explicit conversion.** Measurements are not implicitly normalized to a canonical unit.
- **Predictable arithmetic.** Operations producing lengths preserve the appropriate operand's unit.
- **Data interoperability.** Measurements have a compact, consistent textual representation.
- **Minimalism.** No generic dimensional algebra, custom unit registries, or custom numerical engine.

## Installation

Add the library to your project:

``` sh
cargo add length-rs
```

Or include it in your `Cargo.toml`:

``` toml
[dependencies]
length-rs = "0.1"
```

To enable Serde support:

``` sh
cargo add length-rs --features serde
```

API documentation is available on [docs.rs](<https://docs.rs/length-rs>).

## Examples

### Representing physical lengths

Lengths can be constructed programmatically or parsed from text.

``` rust
use length_rs::{Length, LengthUnit::*};

let width = Length::new(2.625, Inch)?;
let height: Length = "25.4mm".parse()?;

assert_eq!(width.to_string(), "2.625in");
assert_eq!(height.to_string(), "25.4mm");
```

Both values have the same Rust type, despite using different units.

### Converting between units

Conversion is explicit and produces a new length without modifying the original.

``` rust
use length_rs::{Length, LengthUnit::*};

let width: Length = "2.625in".parse()?;
let points = width.convert(Point)?;

assert_eq!(width.unit(), Inch);
assert_eq!(points.unit(), Point);

assert!((points.value() - 189.0).abs() < 1e-12);
```

Numerical values can also be converted directly without constructing a `Length`:

``` rust
use length_rs::LengthUnit::*;

let millimeters = Millimeter.convert(1.0, Inch);
let points = Point.convert(2.625, Inch);
let inches = Inch.convert(72.0, Point);
```

The receiving unit determines the destination of the conversion.

### Selecting units at runtime

Because units are runtime values, they can be selected through configuration, user input, or application logic without changing the type of a measurement.

``` rust
use length_rs::{Length, LengthUnit};

let width: Length = "2.625in".parse()?;
let preferred_unit: LengthUnit = "mm".parse()?;

let converted = width.convert(preferred_unit)?;
```

No generic type parameters, dynamic dispatch, or custom unit registry are required.

### Preserving units through arithmetic

Arithmetic operates naturally across different units of measurement.

``` rust
use length_rs::{Length, LengthError};

fn main() -> Result<(), LengthError> {
    let width: Length = "2.625in".parse()?;
    let margin: Length = "0.125in".parse()?;

    let total = (width + margin)?;
    let inner = (width - margin)?;

    println!("{total}"); // 2.75in
    println!("{inner}"); // 2.5in

    Ok(())
}
```

Addition and subtraction preserve the unit of the left-hand operand.

For example, the following expressions produce physically equivalent results but retain different units:

| Expression | Result |
| --- | --- |
| 1in + 25.4mm | 2in |
| 25.4mm + 1in | 50.8mm |
| 2in − 25.4mm | 1in |
| 50.8mm − 1in | 25.4mm |

Results are subject to floating-point rounding.

This behavior allows applications to control the unit of a result naturally through operand ordering, rather than requiring explicit conversion after every operation.

### Computing physical layouts

Physical layout calculations frequently involve a combination of lengths, scalar quantities, and dimensionless ratios.

Consider a sheet containing three labels, each 2.625 inches wide, separated by two 0.125-inch gutters.

``` rust
use length_rs::{Length, LengthError};

fn main() -> Result<(), LengthError> {
    let width: Length = "2.625in".parse()?;
    let gutter: Length = "0.125in".parse()?;

    let labels = (width * 3.0)?;
    let gutters = (gutter * 2.0)?;
    let total = (labels + gutters)?;

    println!("{total}"); // 8.125in

    Ok(())
}
```

The entire calculation remains expressed in physical lengths, and the result retains the original unit.

Supported arithmetic operators include:

| Expression | Result type |
| --- | --- |
| `length + length` | `Result<Length, LengthError>` |
| `length - length` | `Result<Length, LengthError>` |
| `length * scalar` | `Result<Length, LengthError>` |
| `scalar * length` | `Result<Length, LengthError>` |
| `length / scalar` | `Result<Length, LengthError>` |
| `length / length` | `Result<f64, LengthError>` |
| `-length` | `Length` |

Multiplying two lengths would produce an area, which is intentionally outside the scope of this library.

### Rounding to physical increments

Measurements can be rounded to physical increments rather than merely to a specified number of decimal places.

``` rust
use length_rs::Length;

let width: Length = "2.63in".parse()?;
let increment: Length = "0.125in".parse()?;

let rounded = width.round_to(increment)?;

assert_eq!(rounded.to_string(), "2.625in");
```

The library supports rounding, flooring, and ceiling to positive physical increments, including increments expressed in different units.

This is useful for fabrication tolerances, printing grids, engineering measurements, and alignment.

### Representation versus physical equivalence

Two measurements can represent the same physical distance without having the same representation.

``` rust
use length_rs::Length;

let a: Length = "1in".parse()?;
let b: Length = "25.4mm".parse()?;
let tolerance: Length = "0.000001mm".parse()?;

assert_ne!(a, b);
assert!(a.approx_eq(b, tolerance)?);
```

Ordinary equality compares the stored magnitude and unit. Physical comparison is performed explicitly through `physical_cmp` or `approx_eq`.

This distinction is intentional. Applications can distinguish between measurements as represented and measurements as physical quantities.

### Configuration and data interchange

Unit-bearing values are particularly useful in configuration files, where separating magnitudes from their units introduces unnecessary complexity.

Instead of representing measurements as nested structures:

``` toml
[width]
value = 2.625
unit = "in"

[height]
value = 25.4
unit = "mm"
```

With the optional Serde feature, measurements can be represented directly:

``` toml
width = "2.625in"
height = "25.4mm"
margin = "9pt"
preferred_unit = "mm"
```

The corresponding Rust data model is equally straightforward:

``` rust
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

Each measurement retains its unit throughout deserialization, computation, and subsequent serialization.

There is no requirement to maintain separate unit fields, establish a document-wide measurement system, or convert all values to a common base unit before storing them.

This is particularly useful for measurement catalogs, document templates, paper specifications, and user-editable configuration files.

## Supported units

The library supports 19 absolute physical units spanning four measurement domains.

| Domain | Unit | Symbol |
| --- | --- | --- |
| Metric | Picometer | `pm` |
| Metric | Nanometer | `nm` |
| Metric | Micrometer | `µm` |
| Metric | Millimeter | `mm` |
| Metric | Centimeter | `cm` |
| Metric | Decimeter | `dm` |
| Metric | Meter | `m` |
| Metric | Kilometer | `km` |
| Customary | Mil | `mil` |
| Customary | Inch | `in` |
| Customary | Foot | `ft` |
| Customary | Yard | `yd` |
| Customary | Mile | `mi` |
| Typographic | Point | `pt` |
| Typographic | Pica | `pica` |
| Typographic | Twip | `twip` |
| Astronomical | Astronomical unit | `au` |
| Astronomical | Light-year | `ly` |
| Astronomical | Parsec | `pc` |

All supported units can be enumerated through `LengthUnit::ALL`.

Typographic units use the PostScript convention of 72 points per inch. The symbol `pica` distinguishes the typographic pica from the astronomical parsec (`pc`).

## Text representation

Lengths use a compact, locale-independent textual representation consisting of a signed or unsigned magnitude immediately followed by a unit symbol.

Examples:

``` text
2.625in
25.4mm
-6pt
0m
10µm
1.5pc
1e-9m
```

Unit symbols are case-sensitive. Whitespace, digit separators, omitted units, and non-finite magnitudes are not accepted.

Formatting preserves the stored magnitude and unit, but does not preserve incidental lexical details such as leading zeros, trailing decimal zeros, or exponent notation.

## Features

- Strongly typed, unit-preserving physical lengths.
- Runtime-selectable units.
- Nineteen metric, customary, typographic, and astronomical units.
- Direct conversion of numerical values between units.
- Checked conversion and mixed-unit arithmetic.
- Scalar multiplication and division.
- Dimensionless ratios between lengths.
- Rounding to physical increments.
- Structural equality and explicit physical comparison.
- Strict parsing and consistent formatting.
- Optional Serde serialization and deserialization.
- Zero mandatory external dependencies.
- No unsafe code.

## Numerical behavior

Magnitudes and conversion factors use native Rust `f64` arithmetic.

The library deliberately relies on standard floating-point arithmetic rather than introducing a custom decimal, rational, or arbitrary-precision numerical engine.

The checked `Length` API:

- Accepts finite positive, negative, and zero magnitudes.
- Rejects NaN and infinity.
- Detects arithmetic and conversion overflow.
- Detects underflow when an operation expected to produce a nonzero result rounds to zero.
- Rejects division by zero.
- Rejects invalid rounding increments and negative comparison tolerances.

Ordinary floating-point rounding is expected. Exact decimal arithmetic, correctly rounded conversions across all magnitudes, and exact conversion round trips are not guaranteed.

Intermediate conversions may also encounter numerical limits even when a mathematical result would otherwise be representable.

Direct numerical conversion through `LengthUnit::convert` follows native IEEE 754 floating-point behavior. Use `Length` when checked operations and explicit error reporting are required.

## Scope

The library focuses exclusively on absolute physical lengths.

The following are intentionally outside its scope:

- General-purpose dimensional analysis.
- Areas, volumes, velocities, and other derived quantities.
- Relative or context-dependent units, such as pixels, percentages, and font-relative units.
- User-defined units and conversion registries.
- Compile-time dimensional algebra.
- Arbitrary-precision or exact decimal arithmetic.
- Rendering, layout engines, and device-specific coordinate systems.

The objective is not to provide the largest possible measurement system, but to provide a small, reliable foundation for applications that work with physical lengths.

See [DESIGN.md](<DESIGN.md>) for the complete design specification, including the data model, conversion semantics, arithmetic behavior, and formal text grammar.

## Further examples

The [examples](<examples/>) directory contains runnable demonstrations:

| Example | Description |
| --- | --- |
| [Conversion](<examples/conversion.rs>) | Direct numerical conversion between units |
| [Parsing](<examples/parsing.rs>) | Parsing and formatting physical lengths |
| [Arithmetic](<examples/arithmetic.rs>) | Arithmetic involving physical lengths |
| [Units](<examples/units.rs>) | Enumerating supported units |

Run an example using Cargo:

``` sh
cargo run --example conversion
cargo run --example parsing
cargo run --example arithmetic
cargo run --example units
```

## Requirements

- Rust 1.89 or later.
- Rust 2024 edition.
- No mandatory external dependencies.

Serde support is available through the optional `serde` feature.

## Contributing

Contributions that improve correctness, numerical reliability, documentation, test coverage, and usability are welcome.

Changes should respect the deliberately narrow scope of the library and favor simplicity, predictable behavior, and minimal dependencies.

Before submitting a contribution, verify formatting, linting, tests, and documentation:

``` sh
cargo fmt --all -- --check
cargo clippy --all-targets --no-default-features -- -D warnings
cargo clippy --all-targets --all-features -- -D warnings
cargo test --no-default-features
cargo test --all-features
```

See [CONTRIBUTING.md](<CONTRIBUTING.md>) for development guidelines, verification requirements, and the release process.

## Project resources

- [Source repository](<https://github.com/rbuck/length-rs>)
- [Crates.io](<https://crates.io/crates/length-rs>)
- [API documentation](<https://docs.rs/length-rs>)
- [Design specification](<DESIGN.md>)
- [Examples](<examples/>)
- [Contributing guidelines](<CONTRIBUTING.md>)
- [Issue tracker](<https://github.com/rbuck/length-rs/issues>)

## License

Licensed under the [MIT License](<LICENSE>).
