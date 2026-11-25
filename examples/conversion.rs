use length::LengthUnit::*;

fn main() {
    let mm = Millimeter.convert(2.625, Inch);
    let pt = Point.convert(2.625, Inch);

    println!("{mm} mm");
    println!("{pt} pt");
}
