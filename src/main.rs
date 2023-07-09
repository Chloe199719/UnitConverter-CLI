
use unit_converter::units_type::{self, UnitLength};
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

    let init_unit = units_type::parse_unit(&args[1]).unwrap(); 
    let init_value = args[2].parse::<f64>().unwrap();
    let final_unit = units_type::parse_unit(&args[3]).unwrap();
    if !init_unit.is_same_type( &final_unit) {
        println!("Units must be of the same type");
        return;
    }

    let result = convert_unit(&init_unit, init_value, &final_unit);
    println!("{} {}  =  {} {}", init_value, init_unit.get_name(),  result, final_unit.get_name() );

   
}

fn print_help(){
    todo!("Print help");
    
}

fn convert_unit(init_unit: &units_type::Type, init_value: f64, final_unit: &units_type::Type) -> f64 {
    match init_unit {
        units_type::Type::Length(_) => convert_length(&init_unit, init_value, &final_unit),
        units_type::Type::Volume(_) => convert_volume(&init_unit, init_value, &final_unit),
        units_type::Type::Temperature(_) => convert_temperature(&init_unit, init_value, &final_unit),
        units_type::Type::Weight(_) => convert_weight(&init_unit, init_value, &final_unit),

        
    }
}

fn convert_length(init_unit: &units_type::Type, init_value: f64, final_unit: &units_type::Type) -> f64 {
    let init_unit = match init_unit {
        units_type::Type::Length(unit) => unit,
        _ => panic!("Wrong type"),
    };
    let final_unit = match final_unit {
        units_type::Type::Length(unit) => unit,
        _ => panic!("Wrong type"),
    };
    let init_value = match init_unit {
        UnitLength::Kilometer(base_value) => base_value.value * init_value,
        UnitLength::Hectometer(base_value) => base_value.value * init_value,
        UnitLength::Decameter(base_value) => base_value.value * init_value,
        UnitLength::Meter(base_value) => base_value.value * init_value,
        UnitLength::Decimeter(base_value) => base_value.value * init_value,
        UnitLength::Centimeter(base_value) => base_value.value * init_value,
        UnitLength::Millimeter(base_value) => base_value.value * init_value,
        UnitLength::Feet(base_value) => base_value.value * init_value,
        UnitLength::Inch(base_value) => base_value.value * init_value,
        UnitLength::Yard(base_value) => base_value.value * init_value,
        UnitLength::Mile(base_value) => base_value.value * init_value,
        UnitLength::NauticalMile(base_value) => base_value.value * init_value,
    };
    match final_unit {
        UnitLength::Kilometer(base_value) => init_value / base_value.value,
        UnitLength::Hectometer(base_value) => init_value / base_value.value,
        UnitLength::Decameter(base_value) => init_value / base_value.value,
        UnitLength::Meter(base_value) => init_value / base_value.value,
        UnitLength::Decimeter(base_value) => init_value / base_value.value,
        UnitLength::Centimeter(base_value) => init_value / base_value.value,
        UnitLength::Millimeter(base_value) => init_value / base_value.value,
        UnitLength::Feet(base_value) => init_value / base_value.value,
        UnitLength::Inch(base_value) => init_value / base_value.value,
        UnitLength::Yard(base_value) => init_value / base_value.value,
        UnitLength::Mile(base_value) => init_value / base_value.value,
        UnitLength::NauticalMile(base_value) => init_value / base_value.value,
    }
}

fn convert_volume(init_unit: &units_type::Type, init_value: f64, final_unit: &units_type::Type) -> f64 {
    todo!("Convert volume");
}

fn convert_temperature(init_unit: &units_type::Type, init_value: f64, final_unit: &units_type::Type) -> f64 {
    todo!("Convert temperature");
}

fn convert_weight(init_unit: &units_type::Type, init_value: f64, final_unit: &units_type::Type) -> f64 {
    todo!("Convert mass");
}
