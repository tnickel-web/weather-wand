use super::super::CurrentWeather;
use crate::errors::CustomError;
use serde_json::Value;

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

    let timezone = &parsed_body["timezone"]
        .as_str()
        .ok_or_else(|| Box::new(CustomError::WeatherInfoNotFound("timezone".to_string())))?;

    let current_weather = CurrentWeather {
        temperature: temperature.to_string(),
        windspeed: windspeed.to_string(),
        is_day: is_day.to_string(),
        timestamp: *unix_timestamp,
        timezone: timezone.to_string(),
    };

    Ok(current_weather)
}
