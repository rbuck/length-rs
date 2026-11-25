use length::LengthUnit;

fn main() {
    for unit in LengthUnit::ALL {
        println!("{} ({})", unit.name(), unit.symbol());
    }
}
