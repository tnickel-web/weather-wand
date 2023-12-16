use super::Coordinates;
use super::Location;
use crate::errors::CustomError;
use serde_json::Value;

/// Deserializes a JSON string into a `Location` struct.
///
/// # Arguments
/// * `body`: A `Result` containing a JSON string or an error.
///
/// # Returns
/// Returns a `Result` containing a `Location` instance if deserialization is successful,
/// or an error if the JSON structure is invalid or geolocation information is missing.
///
/// # Errors
/// This function can return errors in the following scenarios:
/// * The JSON string cannot be parsed.
/// * Essential geolocation information (e.g. name, latitude) is not found in the JSON structure.
pub fn deserialize(
    body: Result<String, Box<dyn std::error::Error>>,
) -> Result<Location, Box<dyn std::error::Error>> {
    let parsed_body: Value =
        serde_json::from_str(&body?).map_err(|err| format!("Error parsing JSON: {}", err))?;

    let city_name = &parsed_body["results"][0]["name"]
        .as_str()
        .ok_or_else(|| Box::new(CustomError::GeolocationNotFound("name".to_string())))?;

    let timezone = &parsed_body["results"][0]["timezone"]
        .as_str()
        .ok_or_else(|| Box::new(CustomError::GeolocationNotFound("timezone".to_string())))?;

    let country_code = &parsed_body["results"][0]["country_code"]
        .as_str()
        .ok_or_else(|| Box::new(CustomError::GeolocationNotFound("country_code".to_string())))?;

    let latitude = &parsed_body["results"][0]["latitude"]
        .as_f64()
        .ok_or_else(|| Box::new(CustomError::GeolocationNotFound("latitude".to_string())))?;

    let longitude = &parsed_body["results"][0]["longitude"]
        .as_f64()
        .ok_or_else(|| Box::new(CustomError::GeolocationNotFound("longitude".to_string())))?;

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
