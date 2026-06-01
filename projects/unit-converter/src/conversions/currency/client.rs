use super::unit::CurrencyUnit;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
struct ApiResponse {
    rates: HashMap<String, f64>,
}

pub fn convert(value: f64, from: &CurrencyUnit, to: &CurrencyUnit) -> Result<f64, String> {
    if from == to {
        return Ok(value);
    }

    let url = format!("https://open.er-api.com/v6/latest/{}", from.code());

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))?;

    let response = client
        .get(&url)
        .send()
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("API returned status: {}", response.status()));
    }

    let data: ApiResponse = response
        .json()
        .map_err(|e| format!("Failed to parse API response: {}", e))?;

    let rate = data
        .rates
        .get(to.code())
        .ok_or_else(|| format!("Rate not found for '{}'", to.code()))?;

    Ok(value * rate)
}
