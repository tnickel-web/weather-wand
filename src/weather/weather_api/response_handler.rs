use super::super::CurrentWeather;
use crate::errors::CustomError;
use serde_json::Value;

/// Deserializes a JSON string into a `CurrentWeather` struct.
///
/// # Arguments
/// * `body`: A `Result` containing a JSON string or an error.
///
/// # Returns
/// Returns a `Result` containing a `CurrentWeather` instance if deserialization is successful,
/// or an error if the JSON structure is invalid or essential weather information is missing.
///
/// # Errors
/// This function can return errors in the following scenarios:
/// * The JSON string cannot be parsed.
/// * Essential weather information (e.g. temperature, windspeed) is not found in the JSON structure.
pub fn deserialize(
    body: Result<String, Box<dyn std::error::Error>>,
) -> Result<CurrentWeather, Box<dyn std::error::Error>> {
    let parsed_body: Value =
        serde_json::from_str(&body?).map_err(|err| format!("Error parsing JSON: {}", err))?;

    let temperature = &parsed_body["current_weather"]["temperature"]
        .as_f64()
        .ok_or_else(|| Box::new(CustomError::WeatherInfoNotFound("temperature".to_string())))?;

    let windspeed = &parsed_body["current_weather"]["windspeed"]
        .as_f64()
        .ok_or_else(|| Box::new(CustomError::WeatherInfoNotFound("windspeed".to_string())))?;

    let is_day = &parsed_body["current_weather"]["is_day"]
        .as_u64()
        .ok_or_else(|| Box::new(CustomError::WeatherInfoNotFound("is_day".to_string())))?;

    let unix_timestamp = &parsed_body["current_weather"]["time"]
        .as_u64()
        .ok_or_else(|| Box::new(CustomError::WeatherInfoNotFound("time".to_string())))?;

    let current_weather = CurrentWeather {
        temperature: temperature.to_string(),
        windspeed: windspeed.to_string(),
        is_day: is_day.to_string(),
        timestamp: *unix_timestamp,
    };

    Ok(current_weather)
}
