pub mod response_handler;

use super::Coordinates;
use super::Location;
use std::time::Duration;

pub struct Client {}

impl Client {
    /// Fetches data from the specified URL using an asynchronous HTTP request.
    ///
    /// # Arguments
    /// * `url`: The URL from which to fetch the data.
    ///
    /// # Returns
    /// Returns a `Result` containing a string with the fetched data if successful,
    /// or an error if the request fails, the timeout is exceeded, or the URL is invalid.
    ///
    /// # Panics
    /// Panics if the HTTP request fails and the client is unable to handle the error,
    /// resulting in termination of the program with an error message.
    pub async fn fetch(url: &str) -> Result<String, Box<dyn std::error::Error>> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(2))
            .build()?;

        let response = client.get(url).send().await.unwrap_or_else(|_| {
            println!("Failed to fetch data from the API. You are not connected to the internet or the API is currently not available.");
            std::process::exit(1);
        });

        let body = response.text().await?;

        Ok(body)
    }
}
