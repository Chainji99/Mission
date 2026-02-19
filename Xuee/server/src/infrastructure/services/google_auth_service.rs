use anyhow::{anyhow, Result};
use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use oauth2::reqwest::async_http_client;
use serde::Deserialize;
use std::env;

#[derive(Clone)]
pub struct GoogleAuthService {
    client: BasicClient,
}

#[derive(Debug, Deserialize)]
pub struct GoogleUserInfo {
    pub id: String,
    pub email: String,
    pub verified_email: bool,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
}

impl GoogleAuthService {
    pub fn new() -> Result<Self> {
        let client_id = ClientId::new(
            env::var("GOOGLE_CLIENT_ID").unwrap_or_else(|_| "mock_client_id".to_string()),
        );
        let client_secret = ClientSecret::new(
            env::var("GOOGLE_CLIENT_SECRET").unwrap_or_else(|_| "mock_client_secret".to_string()),
        );
        let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())?;
        let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string())?;
        let redirect_url = RedirectUrl::new(
            env::var("GOOGLE_REDIRECT_URL").unwrap_or_else(|_| "http://localhost:4200/google-callback".to_string()),
        )?;

        let client = BasicClient::new(
            client_id,
            Some(client_secret),
            auth_url,
            Some(token_url),
        )
        .set_redirect_uri(redirect_url);

        Ok(Self { client })
    }

    pub fn get_authorization_url(&self) -> (String, String) {
        let client_id = env::var("GOOGLE_CLIENT_ID").unwrap_or_else(|_| "mock_client_id".to_string());
        
        // Mock Mode for Dev
        if client_id == "your_google_client_id" || client_id == "mock_client_id" {
            let redirect_url = env::var("GOOGLE_REDIRECT_URL")
                .unwrap_or_else(|_| "http://localhost:4200/google-callback".to_string());
            // Return a direct redirect to the frontend callback with a mock code
            return (format!("{}?code=mock_dev_code", redirect_url), "mock_csrf".to_string());
        }

        let (auth_url, csrf_token) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("email".to_string()))
            .add_scope(Scope::new("profile".to_string()))
            .url();

        (auth_url.to_string(), csrf_token.secret().clone())
    }

    pub async fn verify_code(&self, code: String) -> Result<GoogleUserInfo> {
        // Mock Mode for Dev
        if code == "mock_dev_code" {
            return Ok(GoogleUserInfo {
                id: "mock_google_id_12345".to_string(),
                email: "mock_google_user@example.com".to_string(),
                verified_email: true,
                name: "Mock Google User".to_string(),
                given_name: "Mock".to_string(),
                family_name: "User".to_string(),
                picture: "".to_string(),
            });
        }

        let token_response = self
            .client
            .exchange_code(oauth2::AuthorizationCode::new(code))
            .request_async(async_http_client)
            .await
            .map_err(|e| anyhow!("Token exchange failed: {}", e))?;

        let access_token = token_response.access_token().secret();

        let client = reqwest::Client::new();
        let user_info: GoogleUserInfo = client
            .get("https://www.googleapis.com/oauth2/v2/userinfo")
            .bearer_auth(access_token)
            .send()
            .await?
            .json()
            .await?;

        Ok(user_info)
    }
}
