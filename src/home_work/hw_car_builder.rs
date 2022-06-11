#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {
    Manual,
    SemiAuto,
    Automatic
}

struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32
}

pub fn hw_car_builder() {
    println!("Yo!")
}