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

    if parsed_body["error"] == true {
        return Err(Box::new(CustomError::WeatherInfoNotFound(
            "current_weather".to_string(),
        )));
    }

    let temperature = &parsed_body["current_weather"]["temperature"]
        .as_f64()
        .unwrap();
    let windspeed = &parsed_body["current_weather"]["windspeed"]
        .as_f64()
        .unwrap();
    let is_day = &parsed_body["current_weather"]["is_day"].as_u64().unwrap();
    let unix_timestamp = &parsed_body["current_weather"]["time"].as_u64().unwrap();

    let current_weather = CurrentWeather {
        temperature: temperature.to_string(),
        windspeed: windspeed.to_string(),
        is_day: is_day.to_string(),
        timestamp: *unix_timestamp,
    };

    Ok(current_weather)
}

#[cfg(test)]
mod tests {
    use super::deserialize;

    #[test]
    fn deserialize_creates_correct_current_weather_struct() {
        let json =
            r#"{"current_weather":{"time":1702740600,"temperature":8.8,"windspeed":12.7,"is_day":1}}"#.to_string();

        let result = deserialize(Ok(json)).unwrap();

        assert_eq!(result.timestamp, 1702740600);
        assert_eq!(result.temperature, "8.8");
        assert_eq!(result.windspeed, "12.7");
        assert_eq!(result.is_day, "1");
    }

    #[test]
    fn deserialize_throws_error_on_api_error() {
        // The weather API returns an "error" JSON field if there was an error,
        // so we use this to trigger the WeatherInfoNotFound error.
        let result = deserialize(Ok(r#"{"error": true}"#.to_string()));

        assert!(result.is_err());

        if let Some(err) = result.err() {
            assert!(err.to_string().contains("Weather information not found"));
        }
    }
}
