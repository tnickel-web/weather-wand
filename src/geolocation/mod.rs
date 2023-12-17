pub mod geo_api;

use super::config::Config;
use geo_api::response_handler;

/// A struct representing a cities coordinates using `latitude` and `longitude`
pub struct Coordinates {
    pub latitude: String,
    pub longitude: String,
}

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

/// A struct representing a location using `name`, `country_code`, `timezone` and the `Coordinates` struct.
pub struct Location {
    pub name: String,
    pub country_code: String,
    pub timezone: String,
    pub coordinates: Coordinates,
}

/// A struct representing the `url` of the geolocation API.
/// Placeholders in the `url` get replaced using setters.
pub struct GeoApiUrl {
    pub url: String,
}

impl GeoApiUrl {
    /// Instantiate a new `GeoApiUrl` instance.
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

#[cfg(test)]
mod tests {
    use super::{get_info_for, Config, GeoApiUrl};

    #[test]
    fn setters_insert_correct_information_into_url() {
        let mut geo_api_url = GeoApiUrl::new(Config::get_value("geo_api_url").unwrap());

        let expected_url = "https://geocoding-api.open-meteo.com/v1/search?name=New York&count=1&language=en&format=json";

        let actual_url = &geo_api_url.set_location("New York").unwrap().url;

        assert_eq!(actual_url, expected_url);
    }

    #[tokio::test]
    async fn get_info_for_fetches_required_geolocation_information() {
        let result = get_info_for("New York").await.unwrap();

        assert_eq!(result.name, "New York");
        assert_eq!(result.country_code, "US");
        assert_eq!(result.timezone, "America/New_York");
        assert_eq!(result.coordinates.latitude, "40.71427");
        assert_eq!(result.coordinates.longitude, "-74.00597");
    }
}
