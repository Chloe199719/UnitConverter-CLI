#![allow(dead_code)]
pub mod convert_length;
pub mod convert_volume;
pub mod convert_temperature;
pub mod convert_weight;





// Test function
fn convert_to_2digits(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

fn convert_to_3digits(value: f64) -> f64 {
    (value * 1000.0).round() / 1000.0
}