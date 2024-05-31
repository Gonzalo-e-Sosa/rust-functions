// Declare Car struct to describe vehicle with four named fields
#[derive(Debug)]
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (Age, u32),
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

#[derive(Debug)]
enum Age {
    New,
    Used,
}

// Enum to create a Mileage with default value when Empty
#[derive(Debug)]
enum Mileage {
    Empty,
    Is(u32),
}

impl Default for Mileage {
    fn default() -> Self {
        Mileage::Is(0)
    }
}

impl From<u32> for Mileage {
    fn from(value: u32) -> Self {
        Mileage::Is(value)
    }
}

impl Mileage {
    fn to_u32(mileage: &Mileage) -> u32 {
        match mileage {
            Mileage::Empty => 0,
            Mileage::Is(value) => *value,
        }
    }
}

impl PartialEq for Mileage {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Is(l0), Self::Is(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

// Create a car instance
fn car_factory(color: String, motor: Transmission, roof: bool, mileage: Mileage) -> Car {
    // Show details about car order
    // - Check if order is for Used or New car, then check the roof type
    // - Print details for New or Used car based on roof type
    if mileage != Mileage::Empty && mileage != Mileage::Is(0) {
        if roof {
            println!(
                "Prepare a used car: {:?}, {}, Hard top, {} miles\n",
                motor,
                color,
                Mileage::to_u32(&mileage)
            );
        }
    }

    match mileage {
        Mileage::Empty => Car {
            color,
            motor,
            roof,
            age: car_quality(0),
        },
        Mileage::Is(valid_value) => Car {
            color,
            motor,
            roof,
            age: car_quality(valid_value),
        },
    }
}

fn car_quality(miles: u32) -> (Age, u32) {
    if miles == 0 {
        return (Age::New, miles);
    }
    (Age::Used, miles)
}

fn main() {
    /*
    let v1 = [0; 5];
    let v2 = vec![0; 5];
    let mut v3: Vec<u32> = Vec::new();*/

    let colors = ["Blue", "Green", "Red", "Silver"];

    // Declare the car type and initial values
    let mut car: Car;
    let mut engine: Transmission = Transmission::Manual;

    car = car_factory(String::from(colors[2]), engine, true, Mileage::Empty);
    println!(
        "Car order 1: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.age.0, car.roof, car.motor, car.color, car.age.1
    );

    // Car order #2: Used, Semi-automatic, roof
    engine = Transmission::SemiAuto;
    car = car_factory(String::from(colors[3]), engine, false, Mileage::Is(100));
    println!(
        "Car order 2: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.age.0, car.roof, car.motor, car.color, car.age.1
    );

    // Car order #3: Used, Automatic, Hard top
    engine = Transmission::Automatic;
    car = car_factory(String::from(colors[0]), engine, true, Mileage::Is(200));
    println!(
        "Car order 3: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.age.0, car.roof, car.motor, car.color, car.age.1
    );
}
