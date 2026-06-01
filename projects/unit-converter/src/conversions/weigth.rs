use std::str::FromStr;

use crate::conversions::traits::UnitConversion;

pub enum WeightUnit {
    Ton,
    Kilogram,
    Hectogram,
    Decagram,
    Gram,
    Decigram,
    Centigram,
    Miligram,
}

impl UnitConversion for WeightUnit {
    fn to_base(&self, value: f64) -> f64 {
        match self {
            WeightUnit::Ton => value * 1000000.0,
            WeightUnit::Kilogram => value * 1000.0,
            WeightUnit::Hectogram => value * 100.0,
            WeightUnit::Decagram => value * 10.0,
            WeightUnit::Gram => value,
            WeightUnit::Decigram => value * 0.1,
            WeightUnit::Centigram => value * 0.01,
            WeightUnit::Miligram => value * 0.001,
        }
    }

    fn from_base(&self, value: f64) -> f64 {
        match self {
            WeightUnit::Ton => value / 1000000.0,
            WeightUnit::Kilogram => value / 1000.0,
            WeightUnit::Hectogram => value / 100.0,
            WeightUnit::Decagram => value / 10.0,
            WeightUnit::Gram => value,
            WeightUnit::Decigram => value / 0.1,
            WeightUnit::Centigram => value / 0.01,
            WeightUnit::Miligram => value / 0.001,
        }
    }
}

impl FromStr for WeightUnit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "ton" | "T" => Ok(WeightUnit::Ton),
            "kg" | "kilogram" => Ok(WeightUnit::Kilogram),
            "hg" | "hectogram" => Ok(WeightUnit::Hectogram),
            "dag" | "decagram" => Ok(WeightUnit::Decagram),
            "g" | "gram" => Ok(WeightUnit::Gram),
            "dg" | "decigram" => Ok(WeightUnit::Decigram),
            "cg" | "centigram" => Ok(WeightUnit::Centigram),
            "mg" | "miligram" => Ok(WeightUnit::Miligram),
            _ => Err(format!("'{}' it isn't a valid unit ", s)),
        }
    }
}
