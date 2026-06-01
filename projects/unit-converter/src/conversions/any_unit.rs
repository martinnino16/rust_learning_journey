use std::str::FromStr;

use crate::conversions::{
    currency::unit::CurrencyUnit, length::LengthUnit, temperature::TemperatureUnit,
    traits::UnitConversion, weigth::WeightUnit,
};

pub enum AnyUnit {
    Length(LengthUnit),
    Temperature(TemperatureUnit),
    Weigth(WeightUnit),
    Currency(CurrencyUnit),
}

impl FromStr for AnyUnit {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(u) = LengthUnit::from_str(s) {
            return Ok(AnyUnit::Length(u));
        }
        if let Ok(u) = TemperatureUnit::from_str(s) {
            return Ok(AnyUnit::Temperature(u));
        }
        if let Ok(u) = WeightUnit::from_str(s) {
            return Ok(AnyUnit::Weigth(u));
        }
        if let Ok(u) = CurrencyUnit::from_str(s) {
            return Ok(AnyUnit::Currency(u));
        }
        Err(format!("'{}' is not a recognized unit", s))
    }
}

impl AnyUnit {
    pub fn convert(&self, value: f64, to: &AnyUnit) -> Result<f64, String> {
        match (self, to) {
            (AnyUnit::Length(from), AnyUnit::Length(to)) => Ok(from.convert(value, to)),
            (AnyUnit::Temperature(from), AnyUnit::Temperature(to)) => Ok(from.convert(value, to)),
            (AnyUnit::Weigth(from), AnyUnit::Weigth(to)) => Ok(from.convert(value, to)),
            (AnyUnit::Currency(from), AnyUnit::Currency(to)) => {
                crate::conversions::currency::client::convert(value, from, to)
            }
            _ => Err("Cannot convert between unit categories".to_string()),
        }
    }
}
