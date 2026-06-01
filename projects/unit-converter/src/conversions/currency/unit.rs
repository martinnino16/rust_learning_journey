use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum CurrencyUnit {
    Usd,
    Eur,
    Gbp,
    Jpy,
    Clp,
    Mxn,
    Brl,
    Cad,
    Aud,
    Chf,
}

impl CurrencyUnit {
    pub fn code(&self) -> &'static str {
        match self {
            CurrencyUnit::Usd => "USD",
            CurrencyUnit::Eur => "EUR",
            CurrencyUnit::Gbp => "GBP",
            CurrencyUnit::Jpy => "JPY",
            CurrencyUnit::Clp => "CLP",
            CurrencyUnit::Mxn => "MXN",
            CurrencyUnit::Brl => "BRL",
            CurrencyUnit::Cad => "CAD",
            CurrencyUnit::Aud => "AUD",
            CurrencyUnit::Chf => "CHF",
        }
    }
}

impl FromStr for CurrencyUnit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "USD" => Ok(CurrencyUnit::Usd),
            "EUR" => Ok(CurrencyUnit::Eur),
            "GBP" => Ok(CurrencyUnit::Gbp),
            "JPY" => Ok(CurrencyUnit::Jpy),
            "CLP" => Ok(CurrencyUnit::Clp),
            "MXN" => Ok(CurrencyUnit::Mxn),
            "BRL" => Ok(CurrencyUnit::Brl),
            "CAD" => Ok(CurrencyUnit::Cad),
            "AUD" => Ok(CurrencyUnit::Aud),
            "CHF" => Ok(CurrencyUnit::Chf),
            other => Err(format!("'{}' is not a recognized currency", other)),
        }
    }
}
