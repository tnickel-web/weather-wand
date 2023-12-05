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
        .ok_or_else(|| Box::new(CustomError::WeatherInfoNotFound))?;

    let windspeed = &parsed_body["current_weather"]["windspeed"]
        .as_f64()
        .ok_or_else(|| Box::new(CustomError::WeatherInfoNotFound))?;

    let is_day = &parsed_body["current_weather"]["is_day"]
        .as_u64()
        .ok_or_else(|| Box::new(CustomError::WeatherInfoNotFound))?;

    let time = &parsed_body["current_weather"]["time"]
        .as_str()
        .ok_or_else(|| Box::new(CustomError::WeatherInfoNotFound))?;

    let timezone = &parsed_body["timezone"]
        .as_str()
        .ok_or_else(|| Box::new(CustomError::WeatherInfoNotFound))?;

    let current_weather = CurrentWeather {
        temperature: temperature.to_string(),
        windspeed: windspeed.to_string(),
        is_day: is_day.to_string(),
        time: time.to_string(),
        timezone: timezone.to_string(),
    };

    Ok(current_weather)
}
