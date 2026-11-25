use std::cmp::Ordering;

use length::LengthUnit::*;
use length::{Length, LengthError, LengthUnit};

const RELATIVE_TOLERANCE: f64 = 1e-15;

fn length(value: f64, unit: LengthUnit) -> Length {
    Length::new(value, unit).unwrap()
}

fn assert_close(actual: f64, expected: f64) {
    assert!(actual.is_finite());
    assert!(expected.is_finite());

    let scale = actual.abs().max(expected.abs());
    let error = (actual - expected).abs();

    assert!(
        error <= RELATIVE_TOLERANCE * scale,
        "expected {expected:e}, got {actual:e}, error {error:e}"
    );
}

fn assert_length(actual: Length, value: f64, unit: LengthUnit) {
    assert_eq!(actual.unit(), unit);
    assert_close(actual.value(), value);
}

#[test]
fn construction() {
    let value = length(2.625, Inch);

    assert_eq!(value.value(), 2.625);
    assert_eq!(value.unit(), Inch);
}

#[test]
fn construction_nonfinite_values() {
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert_eq!(Length::new(value, Meter), Err(LengthError::NonFiniteValue));
    }
}

#[test]
fn construction_finite_boundaries() {
    let values = [
        f64::MAX,
        f64::MIN,
        f64::MIN_POSITIVE,
        f64::from_bits(1),
        0.0,
        -0.0,
    ];

    for unit in LengthUnit::ALL {
        for value in values {
            let result = length(value, unit);

            assert_eq!(result.value().to_bits(), value.to_bits());
            assert_eq!(result.unit(), unit);
        }
    }
}

#[test]
fn zero() {
    for unit in LengthUnit::ALL {
        let value = Length::zero(unit);

        assert_eq!(value.value(), 0.0);
        assert_eq!(value.unit(), unit);
        assert!(value.is_zero());
        assert!(!value.is_positive());
        assert!(!value.is_negative());
    }
}

#[test]
fn sign_predicates() {
    let positive = length(1.0, Meter);
    let negative = length(-1.0, Meter);
    let zero = length(-0.0, Meter);

    assert!(positive.is_positive());
    assert!(!positive.is_negative());
    assert!(!positive.is_zero());

    assert!(negative.is_negative());
    assert!(!negative.is_positive());
    assert!(!negative.is_zero());

    assert!(zero.is_zero());
    assert!(!zero.is_positive());
    assert!(!zero.is_negative());
}

#[test]
fn parsing_valid_lengths() {
    let cases: [(&str, f64, LengthUnit); 12] = [
        ("2.625in", 2.625, Inch),
        ("25.4mm", 25.4, Millimeter),
        ("-6pt", -6.0, Point),
        ("+6pt", 6.0, Point),
        ("0m", 0.0, Meter),
        ("-0m", -0.0, Meter),
        ("1e-9m", 1e-9, Meter),
        ("1E-9m", 1e-9, Meter),
        ("1e+3m", 1000.0, Meter),
        ("1.5pc", 1.5, Parsec),
        ("10µm", 10.0, Micrometer),
        ("0002.500in", 2.5, Inch),
    ];

    for (input, value, unit) in cases {
        let result = input.parse::<Length>().unwrap();

        assert_eq!(result.value().to_bits(), value.to_bits());
        assert_eq!(result.unit(), unit);
    }
}

#[test]
fn parsing_all_units() {
    for unit in LengthUnit::ALL {
        let input = format!("2.625{unit}");
        let result = input.parse::<Length>().unwrap();

        assert_eq!(result, length(2.625, unit));
    }
}

#[test]
fn parsing_invalid_syntax() {
    let cases = [
        "", "mm", ".5in", "1.in", "--1in", "++1in", "1e", "1e+", "1e-mm", "1e+mm", "NaNmm",
        "infmm", " 1mm",
    ];

    for input in cases {
        assert_eq!(
            input.parse::<Length>(),
            Err(LengthError::InvalidSyntax),
            "unexpected result for {input:?}"
        );
    }
}

#[test]
fn parsing_missing_units() {
    for input in ["1", "-1", "2.625", "1e3"] {
        assert_eq!(input.parse::<Length>(), Err(LengthError::MissingUnit));
    }
}

#[test]
fn parsing_unknown_units() {
    let cases = ["1px", "1MM", "1inch", "1um", "1μm", "1 mm", "1mm "];

    for input in cases {
        assert_eq!(input.parse::<Length>(), Err(LengthError::UnknownUnit));
    }
}

#[test]
fn parsing_overflow() {
    assert_eq!("1e999m".parse::<Length>(), Err(LengthError::Overflow));
}

#[test]
fn parsing_underflow() {
    assert_eq!("1e-999m".parse::<Length>(), Err(LengthError::Underflow));
}

#[test]
fn formatting() {
    let cases = [
        (length(2.625, Inch), "2.625in"),
        (length(25.4, Millimeter), "25.4mm"),
        (length(-6.0, Point), "-6pt"),
        (length(0.0, Meter), "0m"),
        (length(-0.0, Meter), "-0m"),
        (length(10.0, Micrometer), "10µm"),
    ];

    for (value, expected) in cases {
        assert_eq!(value.to_string(), expected);
    }
}

#[test]
fn formatting_canonicalizes_magnitude() {
    let value = "0002.500in".parse::<Length>().unwrap();

    assert_eq!(value.to_string(), "2.5in");
}

#[test]
fn parsing_formatting_round_trip() {
    let values = [
        0.0,
        -0.0,
        1.0,
        -1.0,
        2.625,
        25.4,
        1e-200,
        1e200,
        f64::MIN_POSITIVE,
        f64::from_bits(1),
        f64::MAX,
        f64::MIN,
    ];

    for unit in LengthUnit::ALL {
        for value in values {
            let original = length(value, unit);
            let restored = original.to_string().parse::<Length>().unwrap();

            assert_eq!(restored.unit(), original.unit());
            assert_eq!(restored.value().to_bits(), original.value().to_bits());
        }
    }
}

#[test]
fn conversion() {
    let original = length(2.625, Inch);
    let converted = original.convert(Millimeter).unwrap();

    assert_length(converted, 66.675, Millimeter);
    assert_length(original, 2.625, Inch);
}

#[test]
fn conversion_magnitude_only() {
    let value = length(2.625, Inch);

    assert_close(value.as_unit(Point).unwrap(), 189.0);
    assert_eq!(value.unit(), Inch);
}

#[test]
fn conversion_identity() {
    for unit in LengthUnit::ALL {
        let original = length(f64::MAX, unit);
        let converted = original.convert(unit).unwrap();

        assert_eq!(original, converted);
    }
}

#[test]
fn conversion_overflow() {
    let value = length(f64::MAX, Meter);

    assert_eq!(value.convert(Millimeter), Err(LengthError::Overflow));
}

#[test]
fn conversion_underflow() {
    let value = length(f64::from_bits(1), Millimeter);

    assert_eq!(value.convert(Meter), Err(LengthError::Underflow));
}

#[test]
fn conversion_subnormal() {
    let value = length(f64::MIN_POSITIVE, Millimeter);
    let converted = value.convert(Meter).unwrap();

    assert!(converted.value().is_subnormal());
    assert_eq!(converted.unit(), Meter);
}

#[test]
fn structural_equality() {
    assert_eq!(length(1.0, Inch), length(1.0, Inch));
    assert_ne!(length(1.0, Inch), length(2.0, Inch));
    assert_ne!(length(1.0, Inch), length(25.4, Millimeter));
    assert_eq!(length(0.0, Inch), length(-0.0, Inch));
}

#[test]
fn addition_same_unit() {
    let a = length(2.0, Inch);
    let b = length(3.0, Inch);

    assert_length(a.checked_add(b).unwrap(), 5.0, Inch);
}

#[test]
fn addition_mixed_units() {
    let a = length(1.0, Inch);
    let b = length(25.4, Millimeter);

    assert_length(a.checked_add(b).unwrap(), 2.0, Inch);
    assert_length(b.checked_add(a).unwrap(), 50.8, Millimeter);
}

#[test]
fn addition_preserves_left_hand_unit() {
    let a = length(1.0, Inch);
    let b = length(25.4, Millimeter);

    assert_length((a + b).unwrap(), 2.0, Inch);
    assert_length((b + a).unwrap(), 50.8, Millimeter);
}

#[test]
fn addition_overflow() {
    let value = length(f64::MAX, Meter);

    assert_eq!(value.checked_add(value), Err(LengthError::Overflow));
}

#[test]
fn addition_operator_overflow() {
    let value = length(f64::MAX, Meter);

    assert_eq!(value + value, Err(LengthError::Overflow));
}

#[test]
fn subtraction_same_unit() {
    let a = length(5.0, Inch);
    let b = length(2.0, Inch);

    assert_length(a.checked_sub(b).unwrap(), 3.0, Inch);
}

#[test]
fn subtraction_mixed_units() {
    let a = length(2.0, Inch);
    let b = length(25.4, Millimeter);

    assert_length(a.checked_sub(b).unwrap(), 1.0, Inch);
    assert_length(b.checked_sub(a).unwrap(), -25.4, Millimeter);
}

#[test]
fn subtraction_overflow() {
    let a = length(f64::MAX, Meter);
    let b = length(f64::MIN, Meter);

    assert_eq!(a.checked_sub(b), Err(LengthError::Overflow));
}

#[test]
fn subtraction_to_zero() {
    let value = length(2.625, Inch);

    assert_eq!(value.checked_sub(value).unwrap(), Length::zero(Inch));
}

#[test]
fn subtraction_preserves_left_hand_unit() {
    let a = length(2.0, Inch);
    let b = length(25.4, Millimeter);

    assert_length((a - b).unwrap(), 1.0, Inch);
    assert_length((b - a).unwrap(), -25.4, Millimeter);
}

#[test]
fn subtraction_operator_overflow() {
    let a = length(f64::MAX, Meter);
    let b = length(f64::MIN, Meter);

    assert_eq!(a - b, Err(LengthError::Overflow));
}

#[test]
fn scalar_multiplication() {
    let value = length(2.0, Inch);

    assert_length(value.checked_mul(3.0).unwrap(), 6.0, Inch);
    assert_length(value.checked_mul(-3.0).unwrap(), -6.0, Inch);
    assert_length(value.checked_mul(0.0).unwrap(), 0.0, Inch);
}

#[test]
fn multiplication_overflow() {
    let value = length(f64::MAX, Meter);

    assert_eq!(value.checked_mul(2.0), Err(LengthError::Overflow));
}

#[test]
fn multiplication_underflow() {
    let value = length(f64::from_bits(1), Meter);

    assert_eq!(value.checked_mul(0.25), Err(LengthError::Underflow));
}

#[test]
fn multiplication_operator() {
    let a = length(2.625, Inch);

    assert_length((a * 3.0).unwrap(), 7.875, Inch);
    assert_length((a * -2.0).unwrap(), -5.25, Inch);
    assert_length((a * 0.0).unwrap(), 0.0, Inch);
}

#[test]
fn reverse_multiplication_operator() {
    let a = length(2.625, Inch);

    assert_length((3.0 * a).unwrap(), 7.875, Inch);
    assert_length((-2.0 * a).unwrap(), -5.25, Inch);
    assert_length((0.0 * a).unwrap(), 0.0, Inch);
}

#[test]
fn multiplication_operator_errors() {
    let a = length(f64::MAX, Meter);
    let b = length(f64::from_bits(1), Meter);

    assert_eq!(a * 2.0, Err(LengthError::Overflow));
    assert_eq!(2.0 * a, Err(LengthError::Overflow));

    assert_eq!(b * 0.25, Err(LengthError::Underflow));
    assert_eq!(0.25 * b, Err(LengthError::Underflow));

    assert_eq!(a * f64::NAN, Err(LengthError::NonFiniteValue));
    assert_eq!(f64::NAN * a, Err(LengthError::NonFiniteValue));

    assert_eq!(a * f64::INFINITY, Err(LengthError::NonFiniteValue));
    assert_eq!(f64::INFINITY * a, Err(LengthError::NonFiniteValue));
}

#[test]
fn scalar_division() {
    let value = length(6.0, Inch);

    assert_length(value.checked_div(3.0).unwrap(), 2.0, Inch);
    assert_length(value.checked_div(-3.0).unwrap(), -2.0, Inch);
}

#[test]
fn division_by_zero() {
    let value = length(1.0, Meter);

    for divisor in [0.0, -0.0] {
        assert_eq!(value.checked_div(divisor), Err(LengthError::DivisionByZero));
    }
}

#[test]
fn division_overflow() {
    let value = length(f64::MAX, Meter);

    assert_eq!(value.checked_div(0.5), Err(LengthError::Overflow));
}

#[test]
fn division_underflow() {
    let value = length(f64::from_bits(1), Meter);

    assert_eq!(value.checked_div(4.0), Err(LengthError::Underflow));
}

#[test]
fn division_operator() {
    let a = length(6.0, Inch);

    assert_length((a / 2.0).unwrap(), 3.0, Inch);
    assert_length((a / -2.0).unwrap(), -3.0, Inch);
}

#[test]
fn division_operator_errors() {
    let a = length(f64::MAX, Meter);
    let b = length(f64::from_bits(1), Meter);

    assert_eq!(a / 0.0, Err(LengthError::DivisionByZero));
    assert_eq!(a / -0.0, Err(LengthError::DivisionByZero));

    assert_eq!(a / 0.5, Err(LengthError::Overflow));
    assert_eq!(b / 4.0, Err(LengthError::Underflow));

    assert_eq!(a / f64::NAN, Err(LengthError::NonFiniteValue));
    assert_eq!(a / f64::INFINITY, Err(LengthError::NonFiniteValue));
}

#[test]
fn nonfinite_scalar_operands() {
    let value = length(1.0, Meter);

    for scalar in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert_eq!(value.checked_mul(scalar), Err(LengthError::NonFiniteValue));

        assert_eq!(value.checked_div(scalar), Err(LengthError::NonFiniteValue));
    }
}

#[test]
fn length_ratio() {
    let a = length(2.0, Inch);
    let b = length(25.4, Millimeter);

    assert_close(a.ratio(b).unwrap(), 2.0);
    assert_close(b.ratio(a).unwrap(), 0.5);
}

#[test]
fn length_ratio_zero_divisor() {
    let value = length(1.0, Meter);

    assert_eq!(
        value.ratio(Length::zero(Millimeter)),
        Err(LengthError::DivisionByZero)
    );
}

#[test]
fn negation() {
    let value = length(2.625, Inch);

    assert_eq!(-value, length(-2.625, Inch));
    assert_eq!(-(-value), value);
}

#[test]
fn absolute_value() {
    assert_eq!(length(-2.625, Inch).abs(), length(2.625, Inch));

    assert_eq!(length(2.625, Inch).abs(), length(2.625, Inch));
}

#[test]
fn physical_comparison_same_unit() {
    let a = length(1.0, Meter);
    let b = length(2.0, Meter);

    assert_eq!(a.physical_cmp(b), Ok(Ordering::Less));
    assert_eq!(b.physical_cmp(a), Ok(Ordering::Greater));
    assert_eq!(a.physical_cmp(a), Ok(Ordering::Equal));
}

#[test]
fn physical_comparison_mixed_units() {
    let a = length(1.0, Inch);
    let b = length(50.8, Millimeter);

    assert_eq!(a.physical_cmp(b), Ok(Ordering::Less));
    assert_eq!(b.physical_cmp(a), Ok(Ordering::Greater));
}

#[test]
fn approximate_equality() {
    let a = length(1.0, Inch);
    let b = length(25.4, Millimeter);
    let tolerance = length(1e-12, Meter);

    assert!(a.approx_eq(b, tolerance).unwrap());
    assert!(b.approx_eq(a, tolerance).unwrap());
}

#[test]
fn approximate_inequality() {
    let a = length(1.0, Inch);
    let b = length(30.0, Millimeter);
    let tolerance = length(0.001, Millimeter);

    assert!(!a.approx_eq(b, tolerance).unwrap());
}

#[test]
fn invalid_tolerance() {
    let value = length(1.0, Meter);
    let tolerance = length(-1.0, Millimeter);

    assert_eq!(
        value.approx_eq(value, tolerance),
        Err(LengthError::InvalidTolerance)
    );
}

#[test]
fn rounding_to_increment() {
    let increment = length(0.25, Inch);

    assert_length(length(1.12, Inch).round_to(increment).unwrap(), 1.0, Inch);

    assert_length(length(1.13, Inch).round_to(increment).unwrap(), 1.25, Inch);
}

#[test]
fn rounding_halfway_away_from_zero() {
    let increment = length(1.0, Inch);

    assert_length(length(1.5, Inch).round_to(increment).unwrap(), 2.0, Inch);

    assert_length(length(-1.5, Inch).round_to(increment).unwrap(), -2.0, Inch);
}

#[test]
fn rounding_down() {
    let increment = length(0.25, Inch);

    assert_length(length(1.13, Inch).floor_to(increment).unwrap(), 1.0, Inch);

    assert_length(
        length(-1.13, Inch).floor_to(increment).unwrap(),
        -1.25,
        Inch,
    );
}

#[test]
fn rounding_up() {
    let increment = length(0.25, Inch);

    assert_length(length(1.13, Inch).ceil_to(increment).unwrap(), 1.25, Inch);

    assert_length(length(-1.13, Inch).ceil_to(increment).unwrap(), -1.0, Inch);
}

#[test]
fn rounding_mixed_units() {
    let value = length(1.6, Inch);
    let increment = length(25.4, Millimeter);

    assert_length(value.round_to(increment).unwrap(), 2.0, Inch);
    assert_length(value.floor_to(increment).unwrap(), 1.0, Inch);
    assert_length(value.ceil_to(increment).unwrap(), 2.0, Inch);
}

#[test]
fn rounding_to_zero() {
    let value = length(0.1, Inch);
    let increment = length(1.0, Inch);

    assert_eq!(value.round_to(increment).unwrap(), Length::zero(Inch));
}

#[test]
fn invalid_rounding_increment() {
    let value = length(1.0, Inch);

    for increment in [length(0.0, Inch), length(-1.0, Inch)] {
        assert_eq!(
            value.round_to(increment),
            Err(LengthError::InvalidIncrement)
        );

        assert_eq!(
            value.floor_to(increment),
            Err(LengthError::InvalidIncrement)
        );

        assert_eq!(value.ceil_to(increment), Err(LengthError::InvalidIncrement));
    }
}

#[test]
fn ratio_operator() {
    let a = length(2.0, Inch);
    let b = length(25.4, Millimeter);

    assert_close((a / b).unwrap(), 2.0);
    assert_close((b / a).unwrap(), 0.5);
}

#[test]
fn ratio_operator_signs() {
    let a = length(6.0, Inch);
    let b = length(-2.0, Inch);
    let zero = Length::zero(Inch);

    assert_close((a / b).unwrap(), -3.0);
    assert_close((b / b).unwrap(), 1.0);
    assert_close((zero / a).unwrap(), 0.0);
}

#[test]
fn ratio_operator_errors() {
    let a = length(f64::MAX, Meter);
    let b = length(f64::from_bits(1), Meter);
    let zero = Length::zero(Millimeter);

    assert_eq!(a / zero, Err(LengthError::DivisionByZero));
    assert_eq!(a / b, Err(LengthError::Overflow));
    assert_eq!(b / a, Err(LengthError::Underflow));
}

#[test]
fn arithmetic_operator_equivalence() {
    for unit in LengthUnit::ALL {
        let a = length(6.0, unit);
        let b = length(2.0, unit);

        assert_eq!(a + b, a.checked_add(b));
        assert_eq!(a - b, a.checked_sub(b));
        assert_eq!(a * 2.0, a.checked_mul(2.0));
        assert_eq!(2.0 * a, a.checked_mul(2.0));
        assert_eq!(a / 2.0, a.checked_div(2.0));
        assert_eq!(a / b, a.ratio(b));
    }
}

#[test]
fn label_grid_width() {
    let width = length(2.625, Inch);
    let gutter = length(0.125, Inch);
    let columns = 3.0;

    let labels = (width * columns).unwrap();
    let gutters = (gutter * (columns - 1.0)).unwrap();
    let total = (labels + gutters).unwrap();

    assert_length(total, 8.125, Inch);
}

#[test]
fn proportional_column_layout() {
    let width = length(2.625, Inch);
    let gap = length(0.125, Inch);

    let available = (width - gap).unwrap();
    let left = (available * (2.0 / 3.0)).unwrap();
    let right = (available / 3.0).unwrap();

    let columns = (left + right).unwrap();
    let total = (columns + gap).unwrap();

    assert_length(left, 5.0 / 3.0, Inch);
    assert_length(right, 5.0 / 6.0, Inch);
    assert_length(total, 2.625, Inch);
}
