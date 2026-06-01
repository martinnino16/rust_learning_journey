use std::str::FromStr;

use crate::conversions::traits::UnitConversion;

pub enum LengthUnit {
    Kilometers,
    Miles,
    Meters,
    Feet,
    Hectometers,
    Decameters,
    Decimeters,
    Centimeters,
    Milimiters,
    Inches,
}

impl UnitConversion for LengthUnit {
    fn to_base(&self, value: f64) -> f64 {
        match self {
            LengthUnit::Kilometers => value * 1000.0,
            LengthUnit::Miles => value * 1.609,
            LengthUnit::Meters => value,
            LengthUnit::Feet => value * 0.3048,
            LengthUnit::Hectometers => value * 100.0,
            LengthUnit::Decameters => value * 10.0,
            LengthUnit::Decimeters => value * 0.1,
            LengthUnit::Centimeters => value * 0.01,
            LengthUnit::Milimiters => value * 0.001,
            LengthUnit::Inches => value * 0.254,
        }
    }

    fn from_base(&self, value: f64) -> f64 {
        match self {
            LengthUnit::Kilometers => value / 1000.0,
            LengthUnit::Miles => value / 1.609,
            LengthUnit::Meters => value,
            LengthUnit::Feet => value / 0.3048,
            LengthUnit::Hectometers => value / 100.0,
            LengthUnit::Decameters => value / 10.0,
            LengthUnit::Decimeters => value / 0.1,
            LengthUnit::Centimeters => value / 0.01,
            LengthUnit::Milimiters => value / 0.001,
            LengthUnit::Inches => value / 0.254,
        }
    }
}

impl FromStr for LengthUnit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "kilometers" | "km" => Ok(LengthUnit::Kilometers),
            "miles" | "mi" => Ok(LengthUnit::Miles),
            "meters" | "m" => Ok(LengthUnit::Meters),
            "feet" | "fe" => Ok(LengthUnit::Feet),
            "hectometers" | "hm" => Ok(LengthUnit::Hectometers),
            "decameters" | "dam" => Ok(LengthUnit::Decameters),
            "decimeters" | "dm" => Ok(LengthUnit::Decimeters),
            "centimeters" | "cm" => Ok(LengthUnit::Centimeters),
            "milimiters" | "mm" => Ok(LengthUnit::Milimiters),
            "inches" | "inch" => Ok(LengthUnit::Inches),
            _ => Err(format!("'{}' it isn't a valid unit", s)),
        }
    }
}
