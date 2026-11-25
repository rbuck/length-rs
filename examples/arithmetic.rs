use length::{Length, LengthError, LengthUnit::*};

fn main() -> Result<(), LengthError> {
    let width = Length::new(1.0, Inch)?;
    let margin = Length::new(25.4, Millimeter)?;

    let total = (width + margin)?;

    println!("{width} + {margin} = {total}");

    Ok(())
}
