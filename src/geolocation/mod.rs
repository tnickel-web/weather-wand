pub mod geo_api;

use super::config::Config;
use geo_api::response_handler;

pub async fn get_info_for(location: &str) -> Result<Location, Box<dyn std::error::Error>> {
    let url = GeoApiUrl::set_location(location)?.url;
    let data = geo_api::Client::fetch(url).await;
    let geo_info = response_handler::deserialize(data)?;

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

pub struct GeoApiUrl {
    pub url: String,
}

impl GeoApiUrl {
    pub fn set_location(location: &str) -> Result<GeoApiUrl, Box<dyn std::error::Error>> {
        let url = Config::get_value("geo_api_url")?;
        let new_url = url.replace("__NAME__", location);

        Ok(GeoApiUrl { url: new_url })
    }
}
