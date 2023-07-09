use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();

    


    if args.len() == 1 {
        println!("You must provide a unit and a value");
        println!("-h for help or --help for help");
        return;
    }
    match args[1].as_str() {
        "-h" => {
            print_help();
            return;
            },
        "--help" => {
            print_help();
            return;
        },
        _ => (),
    }


    if args.len() < 4 && args.len() > 2 {
        println!("Usage: {} <unit> <value> <unit_to>", args[0]);
        return;
    }


    println!("Hello, world!");
}

fn print_help(){
    todo!("Print help");
    
}

enum Type {
    Length(UnitLength),
    Weight(UnitWeight),
    Volume(UnitVolume),
    Temperature(UnitTemperature),

}

enum UnitWeight {
    Gram,
    Kilogram,
    Milligram,
    Pound,
    Ounce,
    Stone,
    Ton,
    
}

enum UnitVolume {
    Liter,
    Milliliter,
    FluidOunce,
    Gallon,
    Quart,
    Pint,
    Gil,
}
    
enum UnitTemperature {
    Celsius, 
    Fahrenheit,
    Kelvin, 
}

enum UnitLength {
    Kilometer, // 1000 meter
    Hectometer, // 100 meter
    Decameter, // 10 meter
    Meter, // Base unit
    Decimeter, // 0.1 meter
    Centimeter, // 0.01 meter
    Millimeter, // 0.001 meter
    Feet, // 0.3048 meter
    Inch, // 0.0254 meter
    Yard, // 0.9144 meter
    Mile, // 1609.344 meter
    NauticalMile, // 1852 meter

}
    


fn parse_unit(unit: &str) -> Result<Type, String> {
    let binding = unit.to_lowercase();
    let unit = binding.as_str();
    match unit {
        "m" | "meter" => Ok(Type::Length(UnitLength::Meter)),
        "cm" |"centimeter" => Ok(Type::Length(UnitLength::Centimeter)),
        "mm" | "millimeter" => Ok(Type::Length(UnitLength::Millimeter)),
        "km" | "kilometer" => Ok(Type::Length(UnitLength::Kilometer)),
        "ft" | "feet" => Ok(Type::Length(UnitLength::Feet)),
        "in" | "inch" => Ok(Type::Length(UnitLength::Inch)),
        "yd" | "yard" => Ok(Type::Length(UnitLength::Yard)),
        "mi" | "mile" => Ok(Type::Length(UnitLength::Mile)),

        _ => Err(format!("Unknown unit: {}", unit)),
    }
}