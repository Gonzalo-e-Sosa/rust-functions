// Declare Car struct to describe vehicle with four named fields
#[derive(Debug)]
struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

// Enum to create a Mileage with default value when Empty
enum Mileage {
    Empty,
    Is(u32),
}

impl Default for Mileage {
    fn default() -> Self {
        Mileage::Is(0)
    }
}

// Create a car instance
fn car_factory(
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: Mileage,
) -> Car {
    match mileage {
        Mileage::Empty => Car {
            color,
            transmission,
            convertible,
            mileage: 0,
        },
        Mileage::Is(valid_value) => Car {
            color,
            transmission,
            convertible,
            mileage: valid_value,
        },
    }
}

fn main() {
    let c = car_factory(
        String::from("red"),
        Transmission::Automatic,
        false,
        Mileage::Is(32),
    );

    println!("{:?}", c);
}
