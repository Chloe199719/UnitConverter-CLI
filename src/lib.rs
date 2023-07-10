use convert::{convert_length::convert_length, convert_volume::convert_volume, convert_temperature::convert_temperature, convert_weight::convert_weight};


pub mod units_type;
mod convert;



pub fn convert_unit(init_unit: &units_type::Type, init_value: f64, final_unit: &units_type::Type) -> f64 {
    match init_unit {
        units_type::Type::Length(_) => convert_length(&init_unit, init_value, &final_unit),
        units_type::Type::Volume(_) => convert_volume(&init_unit, init_value, &final_unit),
        units_type::Type::Temperature(_) => convert_temperature(&init_unit, init_value, &final_unit),
        units_type::Type::Weight(_) => convert_weight(&init_unit, init_value, &final_unit),

        
    }
}
