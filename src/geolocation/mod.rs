pub mod geo_api;

use super::config::Config;
use geo_api::response_handler;

/// Asynchronously retrieves location information for a given location string.
///
/// # Arguments
/// * `location`: The location for which to retrieve information.
///
/// # Returns
/// Returns a `Result` containing a `Location` instance if successful,
/// or an error if the retrieval fails, the URL is invalid, or the API response is malformed.
///
/// # Errors
/// This function can return errors in the following scenarios:
/// * The URL construction or modification fails.
/// * The HTTP request to the Geolocation API fails.
/// * Deserialization of the API response into a `Location` struct fails.
pub async fn get_info_for(location: &str) -> Result<Location, Box<dyn std::error::Error>> {
    let base_url = Config::get_value("geo_api_url")?;
    let mut url_unmodified = GeoApiUrl::new(base_url);

    let url = &url_unmodified.set_location(location)?.url;

    let geo_info = response_handler::deserialize(geo_api::Client::fetch(url).await)?;

    Ok(geo_info)
}

/// A struct representing a cities coordinates using `latitude` and `longitude`
pub struct Coordinates {
    pub latitude: String,
    pub longitude: String,
}

/// A struct representing a location using `name`, `country_code`, `timezone` and the `Coordinates` struct.
pub struct Location {
    pub name: String,
    pub country_code: String,
    pub timezone: String,
    pub coordinates: Coordinates,
}

/// A struct representing the `url` of the geolocation API. Placeholders in the `url` get replaced using setters.
pub struct GeoApiUrl {
    pub url: String,
}

impl GeoApiUrl {
    /// Instantiate a new `GeoApiUrl`.
    pub fn new(base_url: String) -> Self {
        GeoApiUrl { url: base_url }
    }

    /// Replaces the city name placeholder in the Geolocation API URL with a value.
    pub fn set_location(
        &mut self,
        location: &str,
    ) -> Result<&mut GeoApiUrl, Box<dyn std::error::Error>> {
        self.url = self.url.replace("__NAME__", location);

        Ok(self)
    }
}
