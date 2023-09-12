pub mod response_handler;

pub struct Client {}

impl Client {
    pub async fn fetch(url: &str) -> Result<std::string::String, reqwest::Error> {
        let response = reqwest::get(url).await?;
        let body = response.text().await?;

        Ok(body)
    }
}
