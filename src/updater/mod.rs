extern crate semver;

use reqwest::header::USER_AGENT;
use semver::Version;
use serde_json::Value;
use super::config::Config;

fn get_current_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

pub fn notify() {
    println!("There is a new version available.");
    println!("Consider updating using weather-wand --update");
}

pub async fn is_new_version_available() -> bool {
    let current_parsed = Version::parse(&get_current_version()).unwrap();

    let latest_version = get_latest_version();
    let latest_parsed = Version::parse(&latest_version.await.unwrap()).unwrap();

    latest_parsed > current_parsed
}

async fn get_latest_version() -> Result<String, Box<dyn std::error::Error>> {
    let api_url = Config::get_value("latest_release_url").unwrap();

    let client = reqwest::Client::new();

    let response = client
        .get(api_url)
        .header(USER_AGENT, "tnickel-web")
        .send()
        .await?;

    let body = response.text().await?;
    let parsed_body: Value = serde_json::from_str(&body)?;

    let latest_version = &parsed_body["tag_name"].to_string();
    let version_parsed = &latest_version[2..latest_version.len() - 1];

    Ok(version_parsed.to_string())
}
