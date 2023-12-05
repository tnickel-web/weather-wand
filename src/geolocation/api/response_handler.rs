use super::Coordinates;
use super::Location;
use crate::errors::CustomError;
use serde_json::Value;

pub fn deserialize(
    body: Result<String, Box<dyn std::error::Error>>,
) -> Result<Location, Box<dyn std::error::Error>> {
    let parsed_body: Value =
        serde_json::from_str(&body?).map_err(|err| format!("Error parsing JSON: {}", err))?;

    println!("{}", parsed_body);

    let city_name = &parsed_body["results"][0]["name"]
        .as_str()
        .ok_or_else(|| Box::new(CustomError::GeolocationNotFound))?
        .to_string();

    let timezone = &parsed_body["results"][0]["timezone"]
        .as_str()
        .ok_or_else(|| Box::new(CustomError::GeolocationNotFound))?
        .to_string();

    let country_code = &parsed_body["results"][0]["country_code"]
        .as_str()
        .ok_or_else(|| Box::new(CustomError::GeolocationNotFound))?
        .to_string();

    let latitude = &parsed_body["results"][0]["latitude"]
        .as_str()
        .ok_or_else(|| Box::new(CustomError::GeolocationNotFound))?
        .to_string();

    let longitude = &parsed_body["results"][0]["longitude"]
        .as_str()
        .ok_or_else(|| Box::new(CustomError::GeolocationNotFound))?
        .to_string();

    let location: Location = Location {
        name: city_name.to_string(),
        country_code: country_code.to_string(),
        timezone: timezone.to_string(),
        coordinates: Coordinates {
            latitude: latitude.to_string(),
            longitude: longitude.to_string(),
        },
    };

    Ok(location)
}
