use super::Coordinates;
use super::Location;
use serde_json::Value;

pub fn deserialize(
    body: Result<String, reqwest::Error>,
) -> Result<Location, Box<dyn std::error::Error>> {
    let parsed_body: Value = serde_json::from_str(&body.unwrap())?;

    let location: Location = Location {
        name: parsed_body["results"][0]["name"].to_string(),
        country_code: parsed_body["results"][0]["country_code"].to_string(),
        timezone: parsed_body["results"][0]["timezone"].to_string(),
        coordinates: Coordinates {
            latitude: parsed_body["results"][0]["latitude"].to_string(),
            longitude: parsed_body["results"][0]["longitude"].to_string(),
        },
    };

    Ok(location)
}
