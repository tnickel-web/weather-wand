pub mod response_handler;

pub struct Client {}

impl Client {
    pub async fn fetch(url: &str) -> Result<String, Box<dyn std::error::Error>> {
        let response = reqwest::get(url).await?;
        let body = response.text().await?;

        Ok(body)
    }
}
