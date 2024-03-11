pub mod weather_api;

use super::config::Config;
use super::geolocation::Coordinates;
use crate::config::args::{TemperatureUnit, WindspeedUnit};
use weather_api::response_handler;

/// A struct representing the current weather using `temperature`, `windspeed`, `is_day` and `timestamp`.
pub struct CurrentWeather {
    pub temperature: String,
    pub windspeed: String,
    pub is_day: String,
    pub timestamp: u64,
}

/// Asynchronously retrieves location information for a given location string.
///
/// # Arguments
/// * `location`: The location for which to retrieve information.
///
/// # Returns
/// Returns a `Result` containing a `CurrentWeather` instance if successful,
/// or an error if the retrieval fails, the URL is invalid, or the API response is malformed.
///
/// # Errors
/// This function can return errors in the following scenarios:
/// * The URL construction or modification fails.
/// * The HTTP request to the Weather API fails.
/// * Deserialization of the API response into a `CurrentWeather` struct fails.
pub async fn get_info_for(
    coordinates: &Coordinates,
    temperature_unit: &TemperatureUnit,
    windspeed_unit: &WindspeedUnit,
) -> Result<CurrentWeather, Box<dyn std::error::Error>> {
    let base_url = Config::get_value("weather_api_url")?;
    let mut url_unmodified = WeatherApiUrl::new(base_url);

    #[cfg(not(tarpaulin_include))] // Disabled due to coverage failing when multi-line
    let url = &url_unmodified
        .set_coordinates(coordinates)?
        .set_temperature_unit(temperature_unit)?
        .set_windspeed_unit(windspeed_unit)?
        .url;

    let weather_info = response_handler::deserialize(weather_api::Client::fetch(url).await)?;

    Ok(weather_info)
}

/// A struct representing the `url` of the weather API.
/// Placeholders in the `url` get replaced using setters.
pub struct WeatherApiUrl {
    pub url: String,
}

impl WeatherApiUrl {
    /// Instantiate a new `WeatherApiUrl` instance.
    pub fn new(base_url: String) -> Self {
        WeatherApiUrl { url: base_url }
    }

    /// Replaces the coordinates placeholder in the Weather API URL with values.
    pub fn set_coordinates(
        &mut self,
        coordinates: &Coordinates,
    ) -> Result<&mut WeatherApiUrl, Box<dyn std::error::Error>> {
        self.url = self
            .url
            .replace("__LAT__", &coordinates.latitude)
            .replace("__LON__", &coordinates.longitude);

        Ok(self)
    }

    /// Replaces the temperature unit placeholder in the Weather API URL with a value.
    pub fn set_temperature_unit(
        &mut self,
        temperature_unit: &TemperatureUnit,
    ) -> Result<&mut WeatherApiUrl, Box<dyn std::error::Error>> {
        self.url = self
            .url
            .replace("__TEMPERATURE_UNIT__", &temperature_unit.to_string());

        Ok(self)
    }

    /// Replaces the wind speed unit placeholder in the Weather API URL with a value.
    pub fn set_windspeed_unit(
        &mut self,
        windspeed_unit: &WindspeedUnit,
    ) -> Result<&mut WeatherApiUrl, Box<dyn std::error::Error>> {
        self.url = self
            .url
            .replace("__WINDSPEED_UNIT__", &windspeed_unit.to_string());

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::config::args::{TemperatureUnit, WindspeedUnit};
    use crate::config::Config;
    use crate::geolocation::Coordinates;
    use crate::weather;
    use crate::weather::WeatherApiUrl;
    use std::time::{SystemTime, UNIX_EPOCH};
    use weather::get_info_for;

    #[test]
    fn setters_insert_correct_information_into_url() {
        let coordinates = Coordinates {
            latitude: "40.71427".to_string(),
            longitude: "-74.00597".to_string(),
        };

        let mut weather_api_url = WeatherApiUrl::new(Config::get_value("weather_api_url").unwrap());

        let actual_url = &weather_api_url
            .set_coordinates(&coordinates)
            .unwrap()
            .set_temperature_unit(&TemperatureUnit::Celsius)
            .unwrap()
            .set_windspeed_unit(&WindspeedUnit::Kmh)
            .unwrap()
            .url;

        assert!(actual_url.contains("latitude=40.71427&longitude=-74.00597"));
        assert!(actual_url.contains("temperature_unit=celsius"));
        assert!(actual_url.contains("windspeed_unit=kmh"));
    }

    #[tokio::test]
    async fn get_info_for_fetches_required_weather_information() {
        let result = get_info_for(
            &Coordinates {
                latitude: "40.71427".to_string(),
                longitude: "-74.00597".to_string(),
            },
            &TemperatureUnit::Celsius,
            &WindspeedUnit::Kmh,
        )
        .await
        .unwrap();

        assert!(!result.temperature.is_empty());
        assert!(!result.windspeed.is_empty());
        assert!(!result.is_day.is_empty());
        assert!(result.timestamp > 0);
        assert!(result.timestamp < current_timestamp());
    }

    fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System Time before UNIX EPOCH.")
            .as_secs()
    }
}
