# length-rs

A lightweight Rust library for representing, converting, and manipulating physical lengths while preserving their units of measurement.

Unlike general-purpose dimensional analysis libraries, `length` retains both the magnitude and the unit as part of the value. Units are selected at runtime rather than through generic type parameters.

## Example

```rust
use length_rs::{Length, LengthUnit::*};

let width: Length = "2.625in".parse()?;

assert_eq!(width.unit(), Inch);
assert_eq!(width.to_string(), "2.625in");

let points = width.convert(Point);

assert_eq!(points.to_string(), "189pt");
```

Convert numerical values directly between units:

```rust
let points = Point.convert(2.625, Inch);
```

Arithmetic ops are natural,

```rust
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

## Features

- Strongly typed, unit-preserving physical lengths.
- Runtime-selectable units.
- Metric, customary, typographic, and astronomical units.
- Simple conversion, arithmetic, and comparison.
- Parsing and formatting of unit-bearing values.
- Optional Serde serialization and deserialization.
- Zero mandatory external dependencies.

## Design

The library focuses exclusively on absolute physical lengths. It does not implement general-purpose dimensional analysis, derived quantities, or custom numerical arithmetic.

Magnitudes and conversions use native `f64` arithmetic and are subject to floating-point rounding.

## License

Licensed under the MIT License.
