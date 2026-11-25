use std::error::Error;

use length::LengthError;

const ERRORS: [LengthError; 10] = [
    LengthError::InvalidSyntax,
    LengthError::InvalidNumber,
    LengthError::MissingUnit,
    LengthError::UnknownUnit,
    LengthError::NonFiniteValue,
    LengthError::Overflow,
    LengthError::Underflow,
    LengthError::DivisionByZero,
    LengthError::InvalidIncrement,
    LengthError::InvalidTolerance,
];

#[test]
fn display_messages() {
    for error in ERRORS {
        let message = error.to_string();

        assert!(
            !message.trim().is_empty(),
            "missing display message for {error:?}"
        );
    }
}

#[test]
fn display_messages_are_distinct() {
    for (i, lhs) in ERRORS.iter().enumerate() {
        for rhs in &ERRORS[i + 1..] {
            assert_ne!(
                lhs.to_string(),
                rhs.to_string(),
                "identical display messages for {lhs:?} and {rhs:?}"
            );
        }
    }
}

#[test]
fn debug_formatting() {
    let cases = [
        (LengthError::InvalidSyntax, "InvalidSyntax"),
        (LengthError::InvalidNumber, "InvalidNumber"),
        (LengthError::MissingUnit, "MissingUnit"),
        (LengthError::UnknownUnit, "UnknownUnit"),
        (LengthError::NonFiniteValue, "NonFiniteValue"),
        (LengthError::Overflow, "Overflow"),
        (LengthError::Underflow, "Underflow"),
        (LengthError::DivisionByZero, "DivisionByZero"),
        (LengthError::InvalidIncrement, "InvalidIncrement"),
        (LengthError::InvalidTolerance, "InvalidTolerance"),
    ];

    for (error, expected) in cases {
        assert_eq!(format!("{error:?}"), expected);
    }
}

#[test]
fn error_trait() {
    for error in ERRORS {
        let standard_error: &dyn Error = &error;

        assert_eq!(standard_error.to_string(), error.to_string());
        assert!(standard_error.source().is_none());
        assert!(standard_error.is::<LengthError>());
    }
}

#[test]
fn equality() {
    for (i, lhs) in ERRORS.iter().enumerate() {
        for (j, rhs) in ERRORS.iter().enumerate() {
            assert_eq!(lhs == rhs, i == j);
        }
    }
}

#[test]
fn copy_semantics() {
    for error in ERRORS {
        let copied = error;

        assert_eq!(error, copied);
    }
}
