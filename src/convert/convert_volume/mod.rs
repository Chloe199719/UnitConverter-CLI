use crate::units_type::{self, UnitVolume};

pub fn convert_volume(init_unit: &units_type::Type, init_value: f64, final_unit: &units_type::Type) -> f64 {
    let init_unit = match init_unit {
        units_type::Type::Volume(unit) => unit,
        _ => panic!("Wrong type"),
    };
    let final_unit = match final_unit {
        units_type::Type::Volume(unit) => unit,
        _ => panic!("Wrong type"),
    };
    let init_value = match init_unit {
        UnitVolume::Liter => UnitVolume::Liter.get_base_value() * init_value,
        UnitVolume::Milliliter => UnitVolume::Milliliter.get_base_value() * init_value,
        UnitVolume::FluidOunce => UnitVolume::FluidOunce.get_base_value() * init_value,
        UnitVolume::Pint => UnitVolume::Pint.get_base_value() * init_value,
        UnitVolume::Gallon => UnitVolume::Gallon.get_base_value() * init_value,
        UnitVolume::Quart => UnitVolume::Quart.get_base_value() * init_value,
        UnitVolume::Gil=> UnitVolume::Gil.get_base_value() * init_value,
    };
    match final_unit {
        UnitVolume::Liter => init_value / UnitVolume::Liter.get_base_value(),
        UnitVolume::Milliliter => init_value / UnitVolume::Milliliter.get_base_value(),
        UnitVolume::FluidOunce => init_value / UnitVolume::FluidOunce.get_base_value(),
        UnitVolume::Pint => init_value / UnitVolume::Pint.get_base_value(),
        UnitVolume::Gallon => init_value / UnitVolume::Gallon.get_base_value(),
        UnitVolume::Quart => init_value / UnitVolume::Quart.get_base_value(),
        UnitVolume::Gil=> init_value / UnitVolume::Gil.get_base_value(),
    }
        
}


#[cfg(test)]

mod test {
    use crate::convert::convert_to_2digits;
    use crate::convert::convert_to_3digits;
    use super::*;
    #[test]
    fn test_l_to_l() {
        let result = convert_volume(&units_type::Type::Volume(UnitVolume::Liter), 1.0, &units_type::Type::Volume(UnitVolume::Liter));
        let result = convert_to_2digits(result);
        assert!(result == 1.0);
    }
    #[test]
    fn test_ml_to_ml() {
        let result = convert_volume(&units_type::Type::Volume(UnitVolume::Milliliter), 1.0, &units_type::Type::Volume(UnitVolume::Milliliter));
        let result = convert_to_2digits(result);
        assert!(result == 1.0);
    }
    #[test]
    fn test_fluidounce_to_fluidounce() {
        let result = convert_volume(&units_type::Type::Volume(UnitVolume::FluidOunce), 1.0, &units_type::Type::Volume(UnitVolume::FluidOunce));
        let result = convert_to_2digits(result);
        assert!(result == 1.0);
    }
    #[test]
    fn test_pint_to_pint() {
        let result = convert_volume(&units_type::Type::Volume(UnitVolume::Pint), 1.0, &units_type::Type::Volume(UnitVolume::Pint));
        let result = convert_to_2digits(result);
        assert!(result == 1.0);
    }
    #[test]
    fn test_gallon_to_gallon() {
        let result = convert_volume(&units_type::Type::Volume(UnitVolume::Gallon), 1.0, &units_type::Type::Volume(UnitVolume::Gallon));
        let result = convert_to_2digits(result);
        assert!(result == 1.0);
    }
    #[test]
    fn test_quart_to_quart() {
        let result = convert_volume(&units_type::Type::Volume(UnitVolume::Quart), 1.0, &units_type::Type::Volume(UnitVolume::Quart));
        let result = convert_to_2digits(result);
        assert!(result == 1.0);
    }
    #[test]
    fn test_gil_to_gil() {
        let result = convert_volume(&units_type::Type::Volume(UnitVolume::Gil), 1.0, &units_type::Type::Volume(UnitVolume::Gil));
        let result = convert_to_2digits(result);
        assert!(result == 1.0);
    }
    #[test]
    fn test_l_to_ml() {
        let result = convert_volume(&units_type::Type::Volume(UnitVolume::Liter), 1.0, &units_type::Type::Volume(UnitVolume::Milliliter));
        let result = convert_to_2digits(result);
        assert!(result == 1000.0);
    }
    #[test]
    fn test_l_to_fluidounce() {
        let result = convert_volume(&units_type::Type::Volume(UnitVolume::Liter), 1.0, &units_type::Type::Volume(UnitVolume::FluidOunce));
        let result = convert_to_2digits(result);
        assert!(result == 33.81);
    }
    #[test]
    fn test_l_to_pint() {
        let result = convert_volume(&units_type::Type::Volume(UnitVolume::Liter), 1.0, &units_type::Type::Volume(UnitVolume::Pint));
        let result = convert_to_2digits(result);
        assert!(result == 2.11);
    }
    #[test]
    fn test_l_to_gallon() {
        let result = convert_volume(&units_type::Type::Volume(UnitVolume::Liter), 1.0, &units_type::Type::Volume(UnitVolume::Gallon));
        let result = convert_to_2digits(result);
        assert!(result == 0.26);
    }
    #[test]
    fn test_l_to_quart() {
        let result = convert_volume(&units_type::Type::Volume(UnitVolume::Liter), 1.0, &units_type::Type::Volume(UnitVolume::Quart));
        let result = convert_to_2digits(result);
        assert!(result == 1.06);
    }
    #[test]
    fn test_l_to_gil() {
        let result = convert_volume(&units_type::Type::Volume(UnitVolume::Liter), 1.0, &units_type::Type::Volume(UnitVolume::Gil));
        let result = convert_to_2digits(result);
        assert!(result == 8.45);
    }
    #[test]
    fn test_ml_to_l() {
        let result = convert_volume(&units_type::Type::Volume(UnitVolume::Milliliter), 1.0, &units_type::Type::Volume(UnitVolume::Liter));
        let result = convert_to_3digits(result);
        assert!(result == 0.001);
    }
    #[test]
    fn test_ml_to_fluidounce() {
        let result = convert_volume(&units_type::Type::Volume(UnitVolume::Milliliter), 1.0, &units_type::Type::Volume(UnitVolume::FluidOunce));
        let result = convert_to_2digits(result);
        assert!(result == 0.03);
    }
    #[test]
    fn test_ml_to_pint() {
        let result = convert_volume(&units_type::Type::Volume(UnitVolume::Milliliter), 1.0, &units_type::Type::Volume(UnitVolume::Pint));
        let result = convert_to_3digits(result);
        assert!(result == 0.002);
    }
    #[test]
    fn test_ml_to_gallon() {
        let result = convert_volume(&units_type::Type::Volume(UnitVolume::Milliliter), 1.0, &units_type::Type::Volume(UnitVolume::Gallon));
        let result = convert_to_2digits(result);
        
        assert!(result == 0.00);
    }
    #[test]
    fn test_ml_to_quart() {
        let result = convert_volume(&units_type::Type::Volume(UnitVolume::Milliliter), 1.0, &units_type::Type::Volume(UnitVolume::Quart));
        let result = convert_to_2digits(result);
        assert!(result == 0.00);
    }
    #[test]
    fn test_ml_to_gil() {
        let result = convert_volume(&units_type::Type::Volume(UnitVolume::Milliliter), 1.0, &units_type::Type::Volume(UnitVolume::Gil));
        let result = convert_to_2digits(result);
        println!("{}", result);
        assert!(result == 0.01);
    }
    #[test]
    fn test_fluidounce_to_l() {
        let result = convert_volume(&units_type::Type::Volume(UnitVolume::FluidOunce), 1.0, &units_type::Type::Volume(UnitVolume::Liter));
        let result = convert_to_2digits(result);
        assert!(result == 0.03);
    }
    #[test]
    fn test_fluidounce_to_ml() {
        let result = convert_volume(&units_type::Type::Volume(UnitVolume::FluidOunce), 1.0, &units_type::Type::Volume(UnitVolume::Milliliter));
        let result = convert_to_2digits(result);
        assert!(result == 29.57);
    }
    #[test]
    fn test_fluidounce_to_pint() {
        let result = convert_volume(&units_type::Type::Volume(UnitVolume::FluidOunce), 1.0, &units_type::Type::Volume(UnitVolume::Pint));
        let result = convert_to_2digits(result);
        assert!(result == 0.06);
    }
    #[test]
    fn test_fluidounce_to_gallon() {
        let result = convert_volume(&units_type::Type::Volume(UnitVolume::FluidOunce), 1.0, &units_type::Type::Volume(UnitVolume::Gallon));
        let result = convert_to_2digits(result);
        assert!(result == 0.01);
    }

}