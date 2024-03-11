pub mod args;

use serde_json::Value;

pub struct Config;

impl Config {
    /// Gets a certain string value from config.json.
    ///
    /// ## Arguments
    /// * `value`: The key to retrieve the corresponding value for.
    ///
    /// # Returns
    /// Returns string value if successful, or an error if the key is not found,
    /// the JSON format is invalid, or the value associated with the key is not a string.
    ///
    /// # Errors
    /// * The JSON file cannot be parsed.
    /// * The specified key is not found in the JSON structure.
    /// * The value associated with the key is not a string.
    pub fn get_value(value: &str) -> Result<String, Box<dyn std::error::Error>> {
        let file_path = include_str!("files/config.json");

        let parsed: Value = serde_json::from_str(file_path)?;

        let json_value = parsed
            .get(value)
            .ok_or_else(|| {
                Box::<dyn std::error::Error>::from(format!(
                    "Invalid JSON format: value '{}' not found",
                    value
                ))
            })?
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| {
                Box::<dyn std::error::Error>::from(format!(
                    "Value for key {} is not a string",
                    value
                ))
            })?;

        Ok(json_value)
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn get_value_returns_correct_geo_api_url() {
        assert_eq!(
            Config::get_value("geo_api_url").unwrap(), 
            "https://geocoding-api.open-meteo.com/v1/search?name=__NAME__&count=1&language=en&format=json"
        );
    }

    #[test]
    fn get_value_returns_correct_weather_api_url() {
        assert_eq!(
            Config::get_value("weather_api_url").unwrap(),
            "https://api.open-meteo.com/v1/forecast?latitude=__LAT__&longitude=__LON__&current_weather=true&temperature_unit=__TEMPERATURE_UNIT__&timezone=auto&windspeed_unit=__WINDSPEED_UNIT__&timeformat=unixtime"
        );
    }

    #[test]
    fn get_value_returns_format_error_on_missing_value() {
        let value = Config::get_value("invalid_api_url");

        assert!(value.is_err());
        assert!(value
            .err()
            .unwrap()
            .to_string()
            .contains("Invalid JSON format"));
    }

    #[test]
    fn get_value_returns_key_error_on_non_string_value() {
        let value = Config::get_value("test_value");

        assert!(value.is_err());
        assert!(value.err().unwrap().to_string().contains("Value for key"));
    }
}
