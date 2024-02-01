use std::time::Duration;

pub mod response_handler;

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

        let response = client.get(url).send().await;

        if response.is_err() {
            #[cfg(not(test))]
            {
                println!("Failed to fetch data from the API. You are not connected to the internet or the API is currently not available.");
                std::process::exit(1);
            }
        }

        let body = response?.text().await?;

        Ok(body)
    }
}

#[cfg(test)]
mod tests {
    use super::Client;
    use httpmock::prelude::*;
    use serde_json::json;
    use std::time::{Duration, SystemTime};

    #[tokio::test]
    async fn fetch_returns_correct_response() {
        let server = MockServer::start();

        let expected_json = json!({
            "current_weather": {
                "time": 0,
                "interval": 0,
                "temperature": 10.5,
                "windspeed": 12.8,
            },
        });

        let weather_mock = server.mock(|when, then| {
            when.method(GET).path("/weather");
            then.status(200).json_body(expected_json.clone());
        });

        let result = Client::fetch(&server.url("/weather")).await;

        let actual_json: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();

        weather_mock.assert();
        assert_eq!(actual_json, expected_json);
    }

    #[tokio::test]
    async fn exceeding_timeout_triggers_error() {
        let start_time = SystemTime::now();
        let two_seconds = Duration::from_secs(2);

        let server = MockServer::start();

        let timeout_mock = server.mock(|when, then| {
            when.method(GET).path("/timeout");
            then.status(200).delay(two_seconds);
        });

        let _response = Client::fetch(&server.url("/timeout")).await;

        timeout_mock.assert();
        assert!(start_time.elapsed().unwrap() > two_seconds);
        assert!(_response.is_err());
    }
}
