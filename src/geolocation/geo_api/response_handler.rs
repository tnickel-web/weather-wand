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
/// * No results are found in the JSON structure.
/// * An error field is found in the JSON structure.
pub fn deserialize(
    body: Result<String, Box<dyn std::error::Error>>,
) -> Result<Location, Box<dyn std::error::Error>> {
    let parsed_body: Value =
        serde_json::from_str(&body?).map_err(|err| format!("Error parsing JSON: {}", err))?;

    if parsed_body["error"] == true || parsed_body["results"].is_null() {
        return Err(Box::new(CustomError::GeolocationNotFound(
            "results".to_string(),
        )));
    }

    let city_name = &parsed_body["results"][0]["name"].as_str().unwrap();
    let timezone = &parsed_body["results"][0]["timezone"].as_str().unwrap();
    let country_code = &parsed_body["results"][0]["country_code"].as_str().unwrap();
    let latitude = &parsed_body["results"][0]["latitude"].as_f64().unwrap();
    let longitude = &parsed_body["results"][0]["longitude"].as_f64().unwrap();
    let region = &parsed_body["results"][0]["admin1"].as_str().unwrap();
    let country = &parsed_body["results"][0]["country"].as_str().unwrap();

    let location: Location = Location {
        name: city_name.to_string(),
        country_code: country_code.to_string(),
        timezone: timezone.to_string(),
        coordinates: Coordinates {
            latitude: latitude.to_string(),
            longitude: longitude.to_string(),
        },
        region: region.to_string(),
        country: country.to_string(),
    };

    Ok(location)
}

#[cfg(test)]
mod tests {
    use super::deserialize;

    #[test]
    fn deserialize_creates_correct_location_struct() {
        let json =
            r#"{"results":[{"name":"New York","latitude":40.71427,"longitude":-74.00597,"country_code":"US","timezone":"America/New_York","country":"United States","admin1":"New York"}]}"#.to_string();

        let result = deserialize(Ok(json)).unwrap();

        assert_eq!(result.name, "New York");
        assert_eq!(result.coordinates.latitude, "40.71427");
        assert_eq!(result.coordinates.longitude, "-74.00597");
        assert_eq!(result.country_code, "US");
        assert_eq!(result.timezone, "America/New_York");
        assert_eq!(result.region, "New York");
        assert_eq!(result.country, "United States");
    }

    #[test]
    fn deserialize_throws_error_on_invalid_city_name() {
        // The geolocation API returns the "generationtime_ms" JSON field if the city name is invalid,
        // so we use this to trigger the GeolocationNotFound error.
        let result = deserialize(Ok(r#"{"generationtime_ms": 0.0}"#.to_string()));

        assert!(result.is_err());

        if let Some(err) = result.err() {
            assert!(err
                .to_string()
                .contains("Geolocation information not found"));
        }
    }
}
