use crate::units_type::{self, UnitTemperature};

pub fn convert_temperature(init_unit: &units_type::Type, init_value: f64, final_unit: &units_type::Type) -> f64 {
    let init_unit = match init_unit {
        units_type::Type::Temperature(unit) => unit,
        _ => panic!("Wrong type"),
    };
    let final_unit = match final_unit {
        units_type::Type::Temperature(unit) => unit,
        _ => panic!("Wrong type"),
    };

    let init_value_kelvin = match init_unit {
        UnitTemperature::Celsius => init_value + 273.15,
        UnitTemperature::Fahrenheit => (init_value - 32.0) * (5.0 / 9.0) + 273.15,
        UnitTemperature::Kelvin => init_value,
    };

    // Convert Kelvin to output unit
    match final_unit {
        UnitTemperature::Celsius => init_value_kelvin - 273.15,
        UnitTemperature::Fahrenheit => (init_value_kelvin - 273.15) * (9.0 / 5.0) + 32.0,
        UnitTemperature::Kelvin => init_value_kelvin,
    }
    
}

#[cfg(test)]
mod test {
    
 

    use crate::convert::convert_to_2digits;

    use super::*;
  
    #[test]

    fn kv_to_c() {
        assert_eq!(convert_to_2digits(convert_temperature(&units_type::Type::Temperature(UnitTemperature::Kelvin), 1.0, &units_type::Type::Temperature(UnitTemperature::Celsius))), -272.15);
    }
    #[test]
    fn kv_to_f() {
        assert_eq!(convert_to_2digits( convert_temperature(&units_type::Type::Temperature(UnitTemperature::Kelvin), 1.0, &units_type::Type::Temperature(UnitTemperature::Fahrenheit))), -457.87);
    }
    #[test]
    fn c_to_kv() {
        assert_eq!(convert_to_2digits( convert_temperature(&units_type::Type::Temperature(UnitTemperature::Celsius), 1.0, &units_type::Type::Temperature(UnitTemperature::Kelvin))), 274.15);
    }
    #[test]
    fn c_to_f() {
        assert_eq!(convert_to_2digits( convert_temperature(&units_type::Type::Temperature(UnitTemperature::Celsius), 1.0, &units_type::Type::Temperature(UnitTemperature::Fahrenheit))), 33.80);
    }
    #[test]
    fn f_to_kv() {
        assert_eq!(convert_to_2digits( convert_temperature(&units_type::Type::Temperature(UnitTemperature::Fahrenheit), 1.0, &units_type::Type::Temperature(UnitTemperature::Kelvin))), 255.93);
    }
    #[test]
    fn f_to_c() {
        assert_eq!(convert_to_2digits( convert_temperature(&units_type::Type::Temperature(UnitTemperature::Fahrenheit), 1.0, &units_type::Type::Temperature(UnitTemperature::Celsius))), -17.22);
    }
    #[test]
    fn f_negative_to_kv() {
        assert_eq!(convert_to_2digits( convert_temperature(&units_type::Type::Temperature(UnitTemperature::Fahrenheit), -1.0, &units_type::Type::Temperature(UnitTemperature::Kelvin))), 254.82);
    }
    #[test]
    fn f_negative_to_c() {
        assert_eq!(convert_to_2digits( convert_temperature(&units_type::Type::Temperature(UnitTemperature::Fahrenheit), -1.0, &units_type::Type::Temperature(UnitTemperature::Celsius))), -18.33);
    }


}