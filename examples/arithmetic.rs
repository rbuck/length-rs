use length::{Length, LengthError, LengthUnit::*};

fn main() -> Result<(), LengthError> {
    // Mixed-unit arithmetic preserves the left-hand unit.
    let width = Length::new(1.0, Inch)?;
    let margin = Length::new(25.4, Millimeter)?;

    let total = (width + margin)?;

    println!("{width} + {margin} = {total}");

    assert_eq!(total.unit(), Inch);

    // Reversing the operands changes the resulting unit.
    let total = (margin + width)?;

    println!("{margin} + {width} = {total}");

    assert_eq!(total.unit(), Millimeter);

    // Calculate the total width of a three-column label layout.
    let width: Length = "2.625in".parse()?;
    let gutter: Length = "0.125in".parse()?;
    let columns = 3.0;

    let labels = (width * columns)?;
    let gutters = (gutter * (columns - 1.0))?;
    let total = (labels + gutters)?;

    println!();
    println!("Label layout:");
    println!("  Columns:      {columns}");
    println!("  Label width:  {width}");
    println!("  Gutter width: {gutter}");
    println!("  Total labels: {labels}");
    println!("  Total gutters: {gutters}");
    println!("  Total width:  {total}");

    assert_eq!(total, Length::new(8.125, Inch)?);

    Ok(())
}
