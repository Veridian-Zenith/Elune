use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub city: String,
    pub country: String,
    pub latitude: f64,
    pub longitude: f64,
}

pub async fn get_location() -> Result<Location, Box<dyn Error>> {
    let client = Client::new();
    let response = client
        .get("http://ip-api.com/json/")
        .send()
        .await?
        .json::<Location>()
        .await?;
    Ok(response)
}
