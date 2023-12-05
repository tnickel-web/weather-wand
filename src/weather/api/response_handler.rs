use super::super::CurrentWeather;
use serde_json::Value;

pub fn deserialize(
    body: Result<std::string::String, Box<dyn std::error::Error>>,
) -> Result<CurrentWeather, Box<dyn std::error::Error>> {
    let base_error_message: String = String::from("Please provide a valid city name and units.");

    let parsed_body: Value =
        serde_json::from_str(&body?).map_err(|err| format!("Error parsing JSON: {}", err))?;

    let temperature = &parsed_body["current_weather"]["temperature"]
        .as_f64()
        .ok_or(format!(
            "Temperature not found in JSON. {}",
            base_error_message
        ))?
        .to_string();

    let windspeed = &parsed_body["current_weather"]["windspeed"]
        .as_f64()
        .ok_or(format!(
            "Wind speed not found in JSON. {}",
            base_error_message
        ))?
        .to_string();

    let is_day = &parsed_body["current_weather"]["is_day"]
        .as_u64()
        .ok_or(format!(
            "Day/Night status not found in JSON. {}",
            base_error_message
        ))?
        .to_string();

    let time = &parsed_body["current_weather"]["time"]
        .as_str()
        .ok_or(format!("Time not found in JSON. {}", base_error_message))?
        .to_string();

    let timezone = &parsed_body["timezone"]
        .as_str()
        .ok_or(format!(
            "Timezone not found in JSON. {}",
            base_error_message
        ))?
        .to_string();

    let current_weather = CurrentWeather {
        temperature: temperature.to_string(),
        windspeed: windspeed.to_string(),
        is_day: is_day.to_string(),
        time: time.to_string(),
        timezone: timezone.to_string(),
    };

    Ok(current_weather)
}
