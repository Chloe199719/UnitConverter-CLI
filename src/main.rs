

use unit_converter::{units_type::{self}, convert_unit};
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
   


    let init_unit = match units_type::parse_unit(&args[1]) {
        Ok(x) => x,
        Err(_) => {
            println!("Invalid unit");
            return;
        },
    };
    let init_value = match args[2].parse::<f64>() {
        Ok(x) => x,
        Err(_) => {
            println!("Invalid value");
            return;
        },
    };
    let final_unit = match units_type::parse_unit(&args[3]) {
        Ok(x) => x,
        Err(_) => {
            println!("Invalid to_unit");
            return;
        },
    };
    if !init_unit.is_same_type( &final_unit) {
        println!("Units must be of the same type");
        return;
    }

    let result = convert_unit(&init_unit, init_value, &final_unit);
    println!("{} {}  =  {} {}", init_value, init_unit.get_name(),  result, final_unit.get_name() );

   
}

fn print_help(){
   println!("Usage: unit_converter <unit> <value> <unit_to>");
    println!("Example: unit_converter 1m 1 cm");
    println!("Available units:");
    println!("Length: km, hm, dam, m, dm, cm, mm, ft, in, yd, mi, nmi");
    println!("Volume: kl, hl, dal, l, dl, cl, ml, qt, pt, gil");
    println!("Temperature: c, f, k");
    println!("Weight: t, kg, hg, dag, g, dg, cg, mg, lb, oz");

    
}


