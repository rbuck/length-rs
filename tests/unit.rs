use length::LengthUnit::*;
use length::{LengthError, LengthUnit};

const RELATIVE_TOLERANCE: f64 = 1e-15;

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

#[test]
fn unit_definitions() {
    let cases = [
        (Picometer, "pm", "picometer", 1e-12),
        (Nanometer, "nm", "nanometer", 1e-9),
        (Micrometer, "µm", "micrometer", 1e-6),
        (Millimeter, "mm", "millimeter", 1e-3),
        (Centimeter, "cm", "centimeter", 1e-2),
        (Decimeter, "dm", "decimeter", 1e-1),
        (Meter, "m", "meter", 1.0),
        (Kilometer, "km", "kilometer", 1e3),
        (Mil, "mil", "mil", 0.000_025_4),
        (Inch, "in", "inch", 0.0254),
        (Foot, "ft", "foot", 0.3048),
        (Yard, "yd", "yard", 0.9144),
        (Mile, "mi", "mile", 1609.344),
        (Point, "pt", "point", 0.000_352_777_777_777_777_8),
        (Pica, "pica", "pica", 0.004_233_333_333_333_333),
        (Twip, "twip", "twip", 0.000_017_638_888_888_888_89),
        (
            AstronomicalUnit,
            "au",
            "astronomical unit",
            149_597_870_700.0,
        ),
        (LightYear, "ly", "light-year", 9_460_730_472_580_800.0),
        (Parsec, "pc", "parsec", 3.085_677_581_491_367e16),
    ];

    assert_eq!(LengthUnit::ALL.len(), cases.len());

    for (unit, symbol, name, meters) in cases {
        assert!(LengthUnit::ALL.contains(&unit));
        assert_eq!(unit.symbol(), symbol);
        assert_eq!(unit.name(), name);
        assert_eq!(unit.to_string(), symbol);
        assert_eq!(symbol.parse::<LengthUnit>(), Ok(unit));

        assert_close(Meter.convert(1.0, unit), meters);
        assert_close(unit.convert(meters, Meter), 1.0);
    }
}

#[test]
fn invalid_symbols() {
    let cases = [
        "", "MM", "In", " mm", "mm ", "meter", "meters", "inch", "inches", "px", "um", "μm",
    ];

    for symbol in cases {
        assert_eq!(
            symbol.parse::<LengthUnit>(),
            Err(LengthError::UnknownUnit),
            "unexpected result for {symbol:?}"
        );
    }
}

#[test]
fn metric_conversions() {
    assert_close(Nanometer.convert(1.0, Picometer), 0.001);
    assert_close(Micrometer.convert(1.0, Nanometer), 0.001);
    assert_close(Millimeter.convert(1.0, Micrometer), 0.001);
    assert_close(Millimeter.convert(1.0, Centimeter), 10.0);
    assert_close(Centimeter.convert(1.0, Decimeter), 10.0);
    assert_close(Millimeter.convert(1.0, Meter), 1000.0);
    assert_close(Meter.convert(1.0, Kilometer), 1000.0);
}

#[test]
fn customary_conversions() {
    assert_close(Mil.convert(1.0, Inch), 1000.0);
    assert_close(Inch.convert(1.0, Foot), 12.0);
    assert_close(Foot.convert(1.0, Yard), 3.0);
    assert_close(Foot.convert(1.0, Mile), 5280.0);
    assert_close(Yard.convert(1.0, Mile), 1760.0);
}

#[test]
fn typographic_conversions() {
    assert_close(Point.convert(1.0, Inch), 72.0);
    assert_close(Pica.convert(1.0, Inch), 6.0);
    assert_close(Twip.convert(1.0, Inch), 1440.0);
    assert_close(Point.convert(1.0, Pica), 12.0);
    assert_close(Twip.convert(1.0, Point), 20.0);
    assert_close(Twip.convert(1.0, Pica), 240.0);
}

#[test]
fn metric_customary_conversions() {
    assert_close(Millimeter.convert(1.0, Inch), 25.4);
    assert_close(Millimeter.convert(1.0, Foot), 304.8);
    assert_close(Millimeter.convert(1.0, Yard), 914.4);
    assert_close(Kilometer.convert(1.0, Mile), 1.609344);
    assert_close(Micrometer.convert(1.0, Mil), 25.4);
}

#[test]
fn practical_conversions() {
    assert_close(Millimeter.convert(2.625, Inch), 66.675);
    assert_close(Point.convert(2.625, Inch), 189.0);
    assert_close(Inch.convert(189.0, Point), 2.625);
    assert_close(Inch.convert(25.4, Millimeter), 1.0);
}

#[test]
fn identity_conversions() {
    let values = [
        0.0,
        -0.0,
        1.0,
        -1.0,
        2.625,
        f64::MIN_POSITIVE,
        f64::from_bits(1),
        f64::MAX,
        f64::MIN,
    ];

    for unit in LengthUnit::ALL {
        for value in values {
            assert_eq!(unit.convert(value, unit).to_bits(), value.to_bits());
        }
    }
}

#[test]
fn all_conversion_pairs() {
    for source in LengthUnit::ALL {
        for target in LengthUnit::ALL {
            let result = target.convert(1.0, source);

            assert!(
                result.is_finite() && result > 0.0,
                "invalid conversion: {source} -> {target}"
            );
        }
    }
}

#[test]
fn all_conversion_round_trips() {
    let values = [
        1e-200, 1e-100, 0.001, 1.0, 2.625, 25.4, 1000.0, 1e100, 1e200,
    ];

    for source in LengthUnit::ALL {
        for target in LengthUnit::ALL {
            for value in values {
                let converted = target.convert(value, source);
                let restored = source.convert(converted, target);

                assert_close(restored, value);
            }
        }
    }
}

#[test]
fn zero_conversions() {
    for source in LengthUnit::ALL {
        for target in LengthUnit::ALL {
            for value in [0.0_f64, -0.0] {
                assert_eq!(target.convert(value, source).to_bits(), value.to_bits());
            }
        }
    }
}

#[test]
fn negative_conversions() {
    for source in LengthUnit::ALL {
        for target in LengthUnit::ALL {
            let positive = target.convert(2.625, source);
            let negative = target.convert(-2.625, source);

            assert_eq!(negative, -positive);
        }
    }
}

#[test]
fn conversion_overflow() {
    assert_eq!(Millimeter.convert(f64::MAX, Meter), f64::INFINITY);
}

#[test]
fn conversion_underflow() {
    assert_eq!(Meter.convert(f64::from_bits(1), Millimeter), 0.0);
}

#[test]
fn conversion_nonfinite_values() {
    assert!(Millimeter.convert(f64::NAN, Meter).is_nan());

    assert_eq!(Millimeter.convert(f64::INFINITY, Meter), f64::INFINITY);

    assert_eq!(
        Millimeter.convert(f64::NEG_INFINITY, Meter),
        f64::NEG_INFINITY
    );
}
