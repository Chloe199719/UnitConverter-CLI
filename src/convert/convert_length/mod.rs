use crate::units_type::{self, UnitLength};

pub fn convert_length(init_unit: &units_type::Type, init_value: f64, final_unit: &units_type::Type) -> f64 {
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

#[cfg(test)]

mod test {
    use super::*;
    use crate::{convert::{convert_to_2digits, convert_to_3digits}, units_type::BaseValue};

    #[test]
    fn convert_km_to_yrd() {
        let init_unit = units_type::Type::Length(UnitLength::Kilometer(BaseValue { value: 1000.0 }));
        let final_unit = units_type::Type::Length(UnitLength::Yard(BaseValue { value: 0.9144 }));
        let init_value = 1.0;
        let final_value = convert_length(&init_unit, init_value, &final_unit);
        assert_eq!(convert_to_2digits(final_value), 1093.61);
    }
    #[test]
    fn convert_yd_to_km(){
        let init_unit = units_type::Type::Length(UnitLength::Yard(BaseValue { value: 0.9144 }));
        let final_unit = units_type::Type::Length(UnitLength::Kilometer(BaseValue { value: 1000.0 }));
        let init_value = 1.0;
        let final_value = convert_length(&init_unit, init_value, &final_unit);
        assert_eq!(convert_to_3digits(final_value), 0.001);
    }
    #[test]
    fn convert_m_to_ft(){
        let init_unit = units_type::Type::Length(UnitLength::Meter(BaseValue { value: 1.0 }));
        let final_unit = units_type::Type::Length(UnitLength::Feet(BaseValue { value: 0.3048 }));
        let init_value = 1.0;
        let final_value = convert_length(&init_unit, init_value, &final_unit);
        assert_eq!(convert_to_2digits(final_value), 3.28);
    }
    #[test]
    fn convert_ft_to_m(){
        let init_unit = units_type::Type::Length(UnitLength::Feet(BaseValue { value: 0.3048 }));
        let final_unit = units_type::Type::Length(UnitLength::Meter(BaseValue { value: 1.0 }));
        let init_value = 1.0;
        let final_value = convert_length(&init_unit, init_value, &final_unit);
        assert_eq!(convert_to_2digits(final_value), 0.3);
    }
    #[test]
    fn convert_m_to_in(){
        let init_unit = units_type::Type::Length(UnitLength::Meter(BaseValue { value: 1.0 }));
        let final_unit = units_type::Type::Length(UnitLength::Inch(BaseValue { value: 0.0254 }));
        let init_value = 1.0;
        let final_value = convert_length(&init_unit, init_value, &final_unit);
        assert_eq!(convert_to_2digits(final_value), 39.37);
    }
    #[test]
    fn convert_in_to_m(){
        let init_unit = units_type::Type::Length(UnitLength::Inch(BaseValue { value: 0.0254 }));
        let final_unit = units_type::Type::Length(UnitLength::Meter(BaseValue { value: 1.0 }));
        let init_value = 1.0;
        let final_value = convert_length(&init_unit, init_value, &final_unit);
        assert_eq!(convert_to_2digits(final_value), 0.03);
    }

    #[test]
    fn convert_mi_to_m(){
        let init_unit = units_type::Type::Length(UnitLength::Mile(BaseValue { value: 1609.344 }));
        let final_unit = units_type::Type::Length(UnitLength::Meter(BaseValue { value: 1.0 }));
        let init_value = 1.0;
        let final_value = convert_length(&init_unit, init_value, &final_unit);
        assert_eq!(convert_to_3digits(final_value), 1609.344);
    }
    #[test]
    fn convert_nm_to_m(){
        let init_unit = units_type::Type::Length(UnitLength::NauticalMile(BaseValue { value: 1852.0 }));
        let final_unit = units_type::Type::Length(UnitLength::Meter(BaseValue { value: 1.0 }));
        let init_value = 1.0;
        let final_value = convert_length(&init_unit, init_value, &final_unit);
        assert_eq!(convert_to_3digits(final_value), 1852.0);
    }
    #[test]
    fn convert_ft_to_in(){
        let init_unit = units_type::Type::Length(UnitLength::Feet(BaseValue { value: 0.3048 }));
        let final_unit = units_type::Type::Length(UnitLength::Inch(BaseValue { value: 0.0254 }));
        let init_value = 1.0;
        let final_value = convert_length(&init_unit, init_value, &final_unit);
       
        assert_eq!(convert_to_2digits(final_value), 12.00);
    }
    #[test]
    fn convert_in_to_ft(){
        let init_unit = units_type::Type::Length(UnitLength::Inch(BaseValue { value: 0.0254 }));
        let final_unit = units_type::Type::Length(UnitLength::Feet(BaseValue { value: 0.3048 }));
        let init_value = 12.0;
        let final_value = convert_length(&init_unit, init_value, &final_unit);
        assert_eq!(convert_to_2digits(final_value), 1.0);
    }
    #[test]
    fn convert_ft_to_mi(){
        let init_unit = units_type::Type::Length(UnitLength::Feet(BaseValue { value: 0.3048 }));
        let final_unit = units_type::Type::Length(UnitLength::Mile(BaseValue { value: 1609.344 }));
        let init_value = 5280.0;
        let final_value = convert_length(&init_unit, init_value, &final_unit);
        assert_eq!(convert_to_3digits(final_value), 1.0);
    }
    #[test]
    fn convert_mi_to_ft(){
        let init_unit = units_type::Type::Length(UnitLength::Mile(BaseValue { value: 1609.344 }));
        let final_unit = units_type::Type::Length(UnitLength::Feet(BaseValue { value: 0.3048 }));
        let init_value = 1.0;
        let final_value = convert_length(&init_unit, init_value, &final_unit);
        assert_eq!(convert_to_3digits(final_value), 5280.0);
    }
    #[test]
    fn convert_ft_to_nm(){
        let init_unit = units_type::Type::Length(UnitLength::Feet(BaseValue { value: 0.3048 }));
        let final_unit = units_type::Type::Length(UnitLength::NauticalMile(BaseValue { value: 1852.0 }));
        let init_value = 6076.115;
        let final_value = convert_length(&init_unit, init_value, &final_unit);
        assert_eq!(convert_to_3digits(final_value), 1.0);
    }
    #[test]
    fn convert_nm_to_ft(){
        let init_unit = units_type::Type::Length(UnitLength::NauticalMile(BaseValue { value: 1852.0 }));
        let final_unit = units_type::Type::Length(UnitLength::Feet(BaseValue { value: 0.3048 }));
        let init_value = 1.0;
        let final_value = convert_length(&init_unit, init_value, &final_unit);
        assert_eq!(convert_to_3digits(final_value), 6076.115);
    }
    #[test]
    fn convert_in_to_mi(){
        let init_unit = units_type::Type::Length(UnitLength::Inch(BaseValue { value: 0.0254 }));
        let final_unit = units_type::Type::Length(UnitLength::Mile(BaseValue { value: 1609.344 }));
        let init_value = 63360.0;
        let final_value = convert_length(&init_unit, init_value, &final_unit);
        assert_eq!(convert_to_3digits(final_value), 1.0);
    }
    #[test]
    fn convert_mi_to_in(){
        let init_unit = units_type::Type::Length(UnitLength::Mile(BaseValue { value: 1609.344 }));
        let final_unit = units_type::Type::Length(UnitLength::Inch(BaseValue { value: 0.0254 }));
        let init_value = 1.0;
        let final_value = convert_length(&init_unit, init_value, &final_unit);
        assert_eq!(convert_to_3digits(final_value), 63360.0);
    }
    #[test]
    fn convert_in_to_nm(){
        let init_unit = units_type::Type::Length(UnitLength::Inch(BaseValue { value: 0.0254 }));
        let final_unit = units_type::Type::Length(UnitLength::NauticalMile(BaseValue { value: 1852.0 }));
        let init_value = 72913.3858;
        let final_value = convert_length(&init_unit, init_value, &final_unit);
        assert_eq!(convert_to_3digits(final_value), 1.0);
    }
    #[test]
    fn convert_mi_to_nm(){
        let init_unit = units_type::Type::Length(UnitLength::Mile(BaseValue { value: 1609.344 }));
        let final_unit = units_type::Type::Length(UnitLength::NauticalMile(BaseValue { value: 1852.0 }));
        let init_value = 1.0;
        let final_value = convert_length(&init_unit, init_value, &final_unit);
      
        assert_eq!(convert_to_3digits(final_value), 0.869);
    }
    
    #[test]
    fn convert_ft_to_in1(){
        let init_unit = units_type::Type::Length(UnitLength::Feet(BaseValue { value: 0.3048 }));
        let final_unit = units_type::Type::Length(UnitLength::Inch(BaseValue { value: 0.0254 }));
        let init_value = 12.0;
        let final_value = convert_length(&init_unit, init_value, &final_unit);
        assert_eq!(convert_to_2digits(final_value), 144.00);
    }
    #[test]
    fn convert_in_to_ft1(){
        let init_unit = units_type::Type::Length(UnitLength::Inch(BaseValue { value: 0.0254 }));
        let final_unit = units_type::Type::Length(UnitLength::Feet(BaseValue { value: 0.3048 }));
        let init_value = 144.0;
        let final_value = convert_length(&init_unit, init_value, &final_unit);
        assert_eq!(final_value, 12.0);
    }
    #[test]
    fn convert_ft_to_mi1(){
        let init_unit = units_type::Type::Length(UnitLength::Feet(BaseValue { value: 0.3048 }));
        let final_unit = units_type::Type::Length(UnitLength::Mile(BaseValue { value: 1609.344 }));
        let init_value = 5280.0;
        let final_value = convert_length(&init_unit, init_value, &final_unit);
        assert_eq!(convert_to_3digits(final_value), 1.0);
    }
    #[test]
    fn convert_mi_to_ft1(){
        let init_unit = units_type::Type::Length(UnitLength::Mile(BaseValue { value: 1609.344 }));
        let final_unit = units_type::Type::Length(UnitLength::Feet(BaseValue { value: 0.3048 }));
        let init_value = 1.0;
        let final_value = convert_length(&init_unit, init_value, &final_unit);
        assert_eq!(convert_to_3digits(final_value), 5280.0);
    }
    #[test]
    fn convert_ft_to_nm2(){
        let init_unit = units_type::Type::Length(UnitLength::Feet(BaseValue { value: 0.3048 }));
        let final_unit = units_type::Type::Length(UnitLength::NauticalMile(BaseValue { value: 1852.0 }));
        let init_value = 6076.115;
        let final_value = convert_length(&init_unit, init_value, &final_unit);
        assert_eq!(convert_to_3digits(final_value), 1.0);
    }
    #[test]
    fn convert_nm_to_ft2(){
        let init_unit = units_type::Type::Length(UnitLength::NauticalMile(BaseValue { value: 1852.0 }));
        let final_unit = units_type::Type::Length(UnitLength::Feet(BaseValue { value: 0.3048 }));
        let init_value = 1.0;
        let final_value = convert_length(&init_unit, init_value, &final_unit);
        assert_eq!(convert_to_3digits(final_value), 6076.115);
    }
    #[test]
    fn convert_ft_to_yd(){
        let init_unit = units_type::Type::Length(UnitLength::Feet(BaseValue { value: 0.3048 }));
        let final_unit = units_type::Type::Length(UnitLength::Yard(BaseValue { value: 0.9144 }));
        let init_value = 3.0;
        let final_value = convert_length(&init_unit, init_value, &final_unit);
        assert_eq!(convert_to_2digits(final_value), 1.00);
    }
    #[test]
    fn convert_yd_to_ft(){
        let init_unit = units_type::Type::Length(UnitLength::Yard(BaseValue { value: 0.9144 }));
        let final_unit = units_type::Type::Length(UnitLength::Feet(BaseValue { value: 0.3048 }));
        let init_value = 1.0;
        let final_value = convert_length(&init_unit, init_value, &final_unit);
        assert_eq!(final_value, 3.0);
    }
    #[test]
    fn convert_yd_to_mi(){
        let init_unit = units_type::Type::Length(UnitLength::Yard(BaseValue { value: 0.9144 }));
        let final_unit = units_type::Type::Length(UnitLength::Mile(BaseValue { value: 1609.344 }));
        let init_value = 1760.0;
        let final_value = convert_length(&init_unit, init_value, &final_unit);
        assert_eq!(convert_to_3digits(final_value), 1.0);
    }
    #[test]
    fn convert_mi_to_yd(){
        let init_unit = units_type::Type::Length(UnitLength::Mile(BaseValue { value: 1609.344 }));
        let final_unit = units_type::Type::Length(UnitLength::Yard(BaseValue { value: 0.9144 }));
        let init_value = 1.0;
        let final_value = convert_length(&init_unit, init_value, &final_unit);
        assert_eq!(convert_to_3digits(final_value), 1760.0);
    }
    #[test]
    fn convert_yd_to_nm(){
        let init_unit = units_type::Type::Length(UnitLength::Yard(BaseValue { value: 0.9144 }));
        let final_unit = units_type::Type::Length(UnitLength::NauticalMile(BaseValue { value: 1852.0 }));
        let init_value = 2025.372;
        let final_value = convert_length(&init_unit, init_value, &final_unit);
        assert_eq!(convert_to_3digits(final_value), 1.0);
    }
    #[test]
    fn convert_nm_to_yd(){
        let init_unit = units_type::Type::Length(UnitLength::NauticalMile(BaseValue { value: 1852.0 }));
        let final_unit = units_type::Type::Length(UnitLength::Yard(BaseValue { value: 0.9144 }));
        let init_value = 1.0;
        let final_value = convert_length(&init_unit, init_value, &final_unit);
        assert_eq!(convert_to_3digits(final_value), 2025.372);
    }
    #[test]
    fn convert_yd_to_m(){
        let init_unit = units_type::Type::Length(UnitLength::Yard(BaseValue { value: 0.9144 }));
        let final_unit = units_type::Type::Length(UnitLength::Meter(BaseValue { value: 1.0 }));
        let init_value = 1.0;
        let final_value = convert_length(&init_unit, init_value, &final_unit);
        assert_eq!(convert_to_3digits(final_value), 0.914);
    }
    #[test]
    fn convert_m_to_yd(){
        let init_unit = units_type::Type::Length(UnitLength::Meter(BaseValue { value: 1.0 }));
        let final_unit = units_type::Type::Length(UnitLength::Yard(BaseValue { value: 0.9144 }));
        let init_value = 0.914;
        let final_value = convert_length(&init_unit, init_value, &final_unit);
        assert_eq!(convert_to_3digits(final_value), 1.0);
    }
    #[test]
    fn convert_yd_to_cm(){
        let init_unit = units_type::Type::Length(UnitLength::Yard(BaseValue { value: 0.9144 }));
        let final_unit = units_type::Type::Length(UnitLength::Centimeter(BaseValue { value: 0.01 }));
        let init_value = 1.0;
        let final_value = convert_length(&init_unit, init_value, &final_unit);
        assert_eq!(final_value, 91.44);
    }

}