use std::str::FromStr;

use crate::conversions::traits::UnitConversion;

pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

impl UnitConversion for TemperatureUnit {
    fn to_base(&self, value: f64) -> f64 {
        match self {
            TemperatureUnit::Kelvin => value,
            TemperatureUnit::Celsius => value + 273.15,
            TemperatureUnit::Fahrenheit => (value - 32.0) * 5.0 / 9.0 + 273.15,
        }
    }

    fn from_base(&self, value: f64) -> f64 {
        match self {
            TemperatureUnit::Kelvin => value,
            TemperatureUnit::Celsius => value - 273.15,
            TemperatureUnit::Fahrenheit => (value - 273.15) * 9.0 / 5.0 + 32.0,
        }
    }
}

impl FromStr for TemperatureUnit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "Celsius" | "celsius" => Ok(TemperatureUnit::Celsius),
            "Fahrenheit" | "fahrenheit" => Ok(TemperatureUnit::Fahrenheit),
            "Kelvin" | "kelvin" => Ok(TemperatureUnit::Kelvin),
            _ => Err(format!("'{}' it isn't a valid unit", s)),
        }
    }
}
