pub mod geo_api;

use super::config::Config;
use geo_api::response_handler;

pub async fn get_info_for(location: &str) -> Result<Location, Box<dyn std::error::Error>> {
    let base_url = Config::get_value("geo_api_url")?;
    let mut url_unmodified = GeoApiUrl::new(base_url);

    let url = &url_unmodified.set_location(location)?.url;

    let geo_info = response_handler::deserialize(geo_api::Client::fetch(url).await)?;

    Ok(geo_info)
}

pub struct Coordinates {
    pub latitude: String,
    pub longitude: String,
}

pub struct Location {
    pub name: String,
    pub country_code: String,
    pub timezone: String,
    pub coordinates: Coordinates,
}

/// A struct representing the `url` of the geolocation API.
/// Placeholder in the `url` get replaced using setters.
pub struct GeoApiUrl {
    pub url: String,
}

impl GeoApiUrl {
    pub fn new(base_url: String) -> Self {
        GeoApiUrl { url: base_url }
    }
    pub fn set_location(
        &mut self,
        location: &str,
    ) -> Result<&mut GeoApiUrl, Box<dyn std::error::Error>> {
        self.url = self.url.replace("__NAME__", location);

        Ok(self)
    }
}
