use length::{Length, LengthError, LengthUnit::*};

fn main() -> Result<(), LengthError> {
    let width: Length = "2.625in".parse()?;

    assert_eq!(width.unit(), Inch);

    let points = width.convert(Point)?;

    println!("{width} = {points}");

    Ok(())
}
