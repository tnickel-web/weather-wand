pub mod weather_api;

use super::config::Config;
use super::geolocation::Coordinates;
use crate::config::args::{TemperatureUnit, WindspeedUnit};
use weather_api::response_handler;

pub struct CurrentWeather {
    pub temperature: String,
    pub windspeed: String,
    pub is_day: String,
    pub timestamp: u64,
    pub timezone: String,
}

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
    pub fn new(base_url: String) -> Self {
        WeatherApiUrl { url: base_url }
    }

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

    pub fn set_temperature_unit(
        &mut self,
        temperature_unit: &TemperatureUnit,
    ) -> Result<&mut WeatherApiUrl, Box<dyn std::error::Error>> {
        self.url = self
            .url
            .replace("__TEMPERATURE_UNIT__", &temperature_unit.to_string());

        Ok(self)
    }

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
