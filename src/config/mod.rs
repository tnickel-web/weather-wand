pub mod args;

use serde_json::Value;

pub struct Config;

impl Config {
    /// Gets a certain string value from config.json.
    ///
    /// ## Arguments
    /// * `value`: The key to retrieve the corresponding value for.
    ///
    /// # Returns
    /// Returns string value if successful, or an error if the key is not found,
    /// the JSON format is invalid, or the value associated with the key is not a string.
    ///
    /// # Errors
    /// * The JSON file cannot be parsed.
    /// * The specified key is not found in the JSON structure.
    /// * The value associated with the key is not a string.
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
