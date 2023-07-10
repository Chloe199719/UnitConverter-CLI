use crate::units_type::{self, UnitWeight};
pub fn convert_weight(init_unit: &units_type::Type, init_value: f64, final_unit: &units_type::Type) -> f64 {
    let init_unit = match init_unit {
        units_type::Type::Weight(unit) => unit,
        _ => panic!("Wrong type"),
    };
    let final_unit = match final_unit {
        units_type::Type::Weight(unit) => unit,
        _ => panic!("Wrong type"),
    };

    let init_value = match init_unit {
        UnitWeight::Kilogram => UnitWeight::Kilogram.get_base_value() * init_value,
        UnitWeight::Gram => UnitWeight::Gram.get_base_value() * init_value,
        UnitWeight::Pound => UnitWeight::Pound.get_base_value() * init_value,
        UnitWeight::Ounce => UnitWeight::Ounce.get_base_value() * init_value,
        UnitWeight::Stone => UnitWeight::Stone.get_base_value() * init_value,
        UnitWeight::Ton => UnitWeight::Ton.get_base_value() * init_value,
        UnitWeight::Milligram => UnitWeight::Milligram.get_base_value() * init_value,
    };
        
    match final_unit {
        UnitWeight::Kilogram => init_value / UnitWeight::Kilogram.get_base_value(),
        UnitWeight::Gram => init_value / UnitWeight::Gram.get_base_value(),
        UnitWeight::Pound => init_value / UnitWeight::Pound.get_base_value(),
        UnitWeight::Ounce => init_value / UnitWeight::Ounce.get_base_value(),
        UnitWeight::Stone => init_value / UnitWeight::Stone.get_base_value(),
        UnitWeight::Ton => init_value / UnitWeight::Ton.get_base_value(),
        UnitWeight::Milligram => init_value / UnitWeight::Milligram.get_base_value(),
    }
}

#[cfg(test)]
mod tests {
    use crate::convert::convert_to_2digits;

    use super::*;
    #[test]
    fn test_kg_to_kg() {
        let result = convert_weight(&units_type::Type::Weight(UnitWeight::Kilogram), 1.0, &units_type::Type::Weight(UnitWeight::Kilogram));
        let result = convert_to_2digits(result);
        assert!(result == 1.0);
    }
    #[test]
    fn test_g_to_g() {
        let result = convert_weight(&units_type::Type::Weight(UnitWeight::Gram), 1.0, &units_type::Type::Weight(UnitWeight::Gram));
        let result = convert_to_2digits(result);
        assert!(result == 1.0);
    }
    #[test]
    fn test_lb_to_lb() {
        let result = convert_weight(&units_type::Type::Weight(UnitWeight::Pound), 1.0, &units_type::Type::Weight(UnitWeight::Pound));
        let result = convert_to_2digits(result);
        assert!(result == 1.0);
    }
    #[test]
    fn test_lb_to_mg() {
        let result = convert_weight(&units_type::Type::Weight(UnitWeight::Pound), 2.2, &units_type::Type::Weight(UnitWeight::Milligram));
        let result = convert_to_2digits(result);
        println!("{}", result);
        assert!(result == 997903.21);
    }
    #[test]
    fn test_mg_to_lb() {
        let result = convert_weight(&units_type::Type::Weight(UnitWeight::Milligram), 997903.21, &units_type::Type::Weight(UnitWeight::Pound));
        let result = convert_to_2digits(result);
        assert!(result == 2.2);
    }

    #[test]
    fn test_kg_to_lb() {
        let result = convert_weight(&units_type::Type::Weight(UnitWeight::Kilogram), 1.0, &units_type::Type::Weight(UnitWeight::Pound));
        let result = convert_to_2digits(result);
        assert!(result == 2.2);
    }

    #[test]
    fn test_lb_to_kg() {
        let result = convert_weight(&units_type::Type::Weight(UnitWeight::Pound), 1.0, &units_type::Type::Weight(UnitWeight::Kilogram));
        let result = convert_to_2digits(result);
        assert!(result == 0.45);
    }

    #[test]
    fn test_g_to_oz() {
        let result = convert_weight(&units_type::Type::Weight(UnitWeight::Gram), 1000.0, &units_type::Type::Weight(UnitWeight::Ounce));
        let result = convert_to_2digits(result);
        assert!(result == 35.27);
    }
    #[test]

    fn test_kg_to_g(){
        let result = convert_weight(&units_type::Type::Weight(UnitWeight::Kilogram), 1.0, &units_type::Type::Weight(UnitWeight::Gram));
        let result = convert_to_2digits(result);
        assert!(result == 1000.0);  
    }

   

    #[test]
    fn test_stone_to_ton() {
        let result = convert_weight(&units_type::Type::Weight(UnitWeight::Stone), 1.0, &units_type::Type::Weight(UnitWeight::Ton));
        let result = convert_to_2digits(result);
        assert!(result == 0.01);
    }
}