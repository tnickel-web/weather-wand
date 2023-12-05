pub mod response_handler;

use super::Coordinates;
use super::Location;

pub struct Client {}

impl Client {
    pub async fn fetch(url: String) -> Result<String, Box<dyn std::error::Error>> {
        let response = reqwest::get(url).await?;
        let body = response.text().await?;

        Ok(body)
    }
}
