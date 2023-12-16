pub mod weather_api;

use super::config::Config;
use super::geolocation::Coordinates;
use crate::config::args::{TemperatureUnit, WindspeedUnit};
use weather_api::response_handler;

/// A struct representing the current weather using `temperature`, `windspeed`, `is_day`, `timestamp` and `timezone`
pub struct CurrentWeather {
    pub temperature: String,
    pub windspeed: String,
    pub is_day: String,
    pub timestamp: u64,
    pub timezone: String,
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

    let url = &url_unmodified
        .set_coordinates(&coordinates.latitude, &coordinates.longitude)?
        .set_temperature_unit(temperature_unit)?
        .set_windspeed_unit(windspeed_unit)?
        .url;

    let weather_info = response_handler::deserialize(weather_api::Client::fetch(url).await)?;

    Ok(weather_info)
}

/// A struct representing the `url` of the weather API.
/// Placeholder in the `url` get replaced using setters.
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
        latitude: &str,
        longitude: &str,
    ) -> Result<&mut WeatherApiUrl, Box<dyn std::error::Error>> {
        self.url = self
            .url
            .replace("__LAT__", latitude)
            .replace("__LON__", longitude);

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
