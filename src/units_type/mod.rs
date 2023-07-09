pub enum Type {
    Length(UnitLength),
    Weight(UnitWeight),
    Volume(UnitVolume),
    Temperature(UnitTemperature),

}
impl Type {
    pub fn is_same_type (&self, other: &Type) -> bool {
        match self {
            Type::Length(_) => {
                match other {
                    Type::Length(_) => true,
                    _ => false,
                }
            },
            Type::Weight(_) => {
                match other {
                    Type::Weight(_) => true,
                    _ => false,
                }
            },
            Type::Volume(_) => {
                match other {
                    Type::Volume(_) => true,
                    _ => false,
                }
            },
            Type::Temperature(_) => {
                match other {
                    Type::Temperature(_) => true,
                    _ => false,
                }
            },
        }
    }
   pub fn get_name(&self) -> String {
        match self {
           
                    Type::Length(UnitLength::Kilometer(_)) => "Kilometer".to_string(),
                    Type::Length(UnitLength::Hectometer(_)) => "Hectometer".to_string(),
                    Type::Length(UnitLength::Decameter(_)) => "Decameter".to_string(),
                    Type::Length(UnitLength::Meter(_)) => "Meter".to_string(),
                    Type::Length(UnitLength::Decimeter(_)) => "Decimeter".to_string(),
                    Type::Length(UnitLength::Centimeter(_)) => "Centimeter".to_string(),
                    Type::Length(UnitLength::Millimeter(_)) => "Millimeter".to_string(),
                    Type::Length(UnitLength::Feet(_)) => "Feet".to_string(),
                    Type::Length(UnitLength::Inch(_)) => "Inch".to_string(),
                    Type::Length(UnitLength::Yard(_)) => "Yard".to_string(),
                    Type::Length(UnitLength::Mile(_)) => "Mile".to_string(),
                    Type::Length(UnitLength::NauticalMile(_)) => "NauticalMile".to_string(),
                    Type::Weight(UnitWeight::Kilogram) => "Kilogram".to_string(),
                    Type::Weight(UnitWeight::Gram) => "Gram".to_string(),
                    Type::Weight(UnitWeight::Milligram) => "Milligram".to_string(),
                    Type::Weight(UnitWeight::Pound) => "Pound".to_string(),
                    Type::Weight(UnitWeight::Ounce) => "Ounce".to_string(),
                    Type::Weight(UnitWeight::Stone) => "Stone".to_string(),
                    Type::Weight(UnitWeight::Ton) => "Ton".to_string(),
                    Type::Volume(UnitVolume::Liter) => "Liter".to_string(),
                    Type::Volume(UnitVolume::Milliliter) => "Milliliter".to_string(),
                    Type::Volume(UnitVolume::FluidOunce) => "FluidOunce".to_string(),
                    Type::Volume(UnitVolume::Gallon) => "Gallon".to_string(),
                    Type::Volume(UnitVolume::Quart) => "Quart".to_string(),
                    Type::Volume(UnitVolume::Pint) => "Pint".to_string(),
                    Type::Volume(UnitVolume::Gil) => "Gil".to_string(),
                    Type::Temperature(UnitTemperature::Celsius) => "Celsius".to_string(),
                    Type::Temperature(UnitTemperature::Fahrenheit) => "Fahrenheit".to_string(),
                    Type::Temperature(UnitTemperature::Kelvin) => "Kelvin".to_string(),

            
            
        }
    }
}

pub enum UnitWeight {
    Gram,
    Kilogram,
    Milligram,
    Pound,
    Ounce,
    Stone,
    Ton,
    
}

pub enum UnitVolume {
    Liter,
    Milliliter,
    FluidOunce,
    Gallon,
    Quart,
    Pint,
    Gil,
}
    
pub enum UnitTemperature {
    Celsius, 
    Fahrenheit,
    Kelvin, 
}
pub struct BaseValue {
    pub value: f64,
}

pub enum UnitLength {
    Kilometer(BaseValue), // 1000 meter
    Hectometer(BaseValue), // 100 meter
    Decameter(BaseValue), // 10 meter
    Meter(BaseValue), // Base unit
    Decimeter(BaseValue), // 0.1 meter
    Centimeter(BaseValue), // 0.01 meter
    Millimeter(BaseValue), // 0.001 meter
    Feet(BaseValue), // 0.3048 meter
    Inch(BaseValue), // 0.0254 meter
    Yard(BaseValue), // 0.9144 meter
    Mile(BaseValue), // 1609.344 meter
    NauticalMile(BaseValue), // 1852 meter

}



pub fn parse_unit(unit: &str) -> Result<Type, String> {
    let binding = unit.to_lowercase();
    let unit = binding.as_str();
    match unit {
        "m" | "meter" => Ok(Type::Length(UnitLength::Meter(BaseValue{value: 1.0}))),
        "hc" | "hectometer" => Ok(Type::Length(UnitLength::Hectometer(BaseValue{value: 100.0}))),
        "dam" | "decameter" => Ok(Type::Length(UnitLength::Decameter(BaseValue{value: 10.0}))),
        "dm" | "decimeter" => Ok(Type::Length(UnitLength::Decimeter(BaseValue{value: 0.1}))),
        "cm" |"centimeter" => Ok(Type::Length(UnitLength::Centimeter(BaseValue{value: 0.01}))),
        "mm" | "millimeter" => Ok(Type::Length(UnitLength::Millimeter(BaseValue{value: 0.001}))),
        "km" | "kilometer" => Ok(Type::Length(UnitLength::Kilometer(BaseValue{value: 1000.0}))),
        "ft" | "feet" => Ok(Type::Length(UnitLength::Feet(BaseValue{value: 0.3048}))),
        "in" | "inch" => Ok(Type::Length(UnitLength::Inch(BaseValue { value: 0.0254 }))),
        "yd" | "yard" => Ok(Type::Length(UnitLength::Yard(BaseValue { value: 0.9144 }))),
        "mi" | "mile" => Ok(Type::Length(UnitLength::Mile(BaseValue { value: 1609.344 }))),
        "nmi" | "nauticalmile" => Ok(Type::Length(UnitLength::NauticalMile(BaseValue { value: 1852.0 }))),
        "kg" | "kilogram" => Ok(Type::Weight(UnitWeight::Kilogram)),
        "g" | "gram" => Ok(Type::Weight(UnitWeight::Gram)),
        "mg" | "milligram" => Ok(Type::Weight(UnitWeight::Milligram)),
        "lb" | "pound" => Ok(Type::Weight(UnitWeight::Pound)),
        "oz" | "ounce" => Ok(Type::Weight(UnitWeight::Ounce)),
        "st" | "stone" => Ok(Type::Weight(UnitWeight::Stone)),
        "t" | "ton" => Ok(Type::Weight(UnitWeight::Ton)),
        "l" | "liter" => Ok(Type::Volume(UnitVolume::Liter)),
        "ml" | "milliliter" => Ok(Type::Volume(UnitVolume::Milliliter)),
        "floz" | "fluidounce" => Ok(Type::Volume(UnitVolume::FluidOunce)),
        "gal" | "gallon" => Ok(Type::Volume(UnitVolume::Gallon)),
        "qt" | "quart" => Ok(Type::Volume(UnitVolume::Quart)),
        "pt" | "pint" => Ok(Type::Volume(UnitVolume::Pint)),
        "gil"  => Ok(Type::Volume(UnitVolume::Gil)),
        "c" | "celsius" => Ok(Type::Temperature(UnitTemperature::Celsius)),
        "f" | "fahrenheit" => Ok(Type::Temperature(UnitTemperature::Fahrenheit)),  
        "k" | "kelvin" => Ok(Type::Temperature(UnitTemperature::Kelvin)),


        _ => Err(format!("Unknown unit: {}", unit)),
    }
}