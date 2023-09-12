pub mod args;

use serde_json::Value;

pub struct Config;

impl Config {
    pub fn get_value(value: &str) -> Result<String, Box<dyn std::error::Error>> {
        let file_path = include_str!("files/config.json");

        let parsed: Value = serde_json::from_str(file_path)?;
        let json_value = parsed[value]
            .as_str()
            .ok_or(format!("Invalid JSON format: {} not found", value))?
            .to_string();

        Ok(json_value)
    }
}
