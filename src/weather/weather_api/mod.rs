use std::time::Duration;

pub mod response_handler;

pub struct Client {}

impl Client {
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
