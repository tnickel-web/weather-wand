pub mod args;

use serde_json::Value;

pub struct Config;

impl Config {
    pub fn get_value(value: &str) -> Result<String, Box<dyn std::error::Error>> {
        let file_path = include_str!("files/config.json");

        let parsed: Value = serde_json::from_str(file_path)?;

        let json_value = parsed
            .get(value)
            .ok_or_else(|| {
                Box::<dyn std::error::Error>::from(format!(
                    "Invalid JSON format: value '{}' not found",
                    value
                ))
            })?
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| {
                Box::<dyn std::error::Error>::from(format!(
                    "Value for key {} is not a string",
                    value
                ))
            })?;

        Ok(json_value)
    }
}
