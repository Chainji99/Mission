use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use sha1::{Digest, Sha1};

use crate::config::config_loader::get_cloudinary_env;

#[derive(Debug, Deserialize)]
struct CloudinaryResponse {
    secure_url: String,
}

pub struct ImageStorageService;

impl ImageStorageService {
    pub async fn upload(base64_image: &str) -> Result<String> {
        let config = get_cloudinary_env().map_err(|e| anyhow!("Failed to load Cloudinary config: {}", e))?;
        
        let client = Client::new();
        let url = format!(
            "https://api.cloudinary.com/v1_1/{}/image/upload",
            config.cloud_name
        );

        let timestamp = chrono::Utc::now().timestamp().to_string();
        
        let string_to_sign = format!("timestamp={}{}", timestamp, config.api_secret);
        
        let signature = {
            let mut hasher = Sha1::new();
            hasher.update(string_to_sign.as_bytes());
            hex::encode(hasher.finalize())
        };

        let params = json!({
            "file": base64_image,
            "api_key": config.api_key,
            "timestamp": timestamp,
            "signature": signature
        });

        let response = client
            .post(&url)
            .json(&params)
            .send()
            .await
            .map_err(|e| anyhow!("Failed to send request to Cloudinary: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!("Cloudinary upload failed: {}", error_text));
        }

        let cloudinary_response: CloudinaryResponse = response
            .json()
            .await
            .map_err(|e| anyhow!("Failed to parse Cloudinary response: {}", e))?;

        Ok(cloudinary_response.secure_url)
    }
}
