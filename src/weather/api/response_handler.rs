use super::super::CurrentWeather;
use serde_json::Value;

pub fn deserialize(
    body: Result<std::string::String, reqwest::Error>,
) -> Result<CurrentWeather, Box<dyn std::error::Error>> {
    let parsed_body: Value = serde_json::from_str(&body.unwrap())?;

    let temperature = &parsed_body["current_weather"]["temperature"];
    let windspeed = &parsed_body["current_weather"]["windspeed"];

    let is_day = &parsed_body["current_weather"]["is_day"];
    let time = &parsed_body["current_weather"]["time"];
    let timezone = &parsed_body["timezone"];

    let current_weather = CurrentWeather {
        temperature: temperature.to_string(),
        windspeed: windspeed.to_string(),

        is_day: is_day.to_string(),
        time: time.to_string(),
        timezone: timezone.to_string(),
    };

    Ok(current_weather)
}
