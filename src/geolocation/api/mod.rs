pub mod response_handler;

use super::Coordinates;
use super::Location;

pub struct Client {}

impl Client {
    pub async fn fetch(url: String) -> Result<std::string::String, reqwest::Error> {
        let response = reqwest::get(url).await?;

        response.text().await
    }
}
