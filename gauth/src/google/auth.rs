use axum::extract::{Query, State};
use serde::Deserialize;

use crate::db::Sqlite;
use crate::models::User;

use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct Token {
    access_token: String,
}

#[derive(Debug, Deserialize)]
pub struct TokenError {
    error: String,
    error_description: Option<String>,
}

#[derive(Clone)]
pub struct GoogleAuth {
    pub sqlite: Arc<Sqlite>,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

impl GoogleAuth {
    pub fn new(sqlite: Sqlite, client_id: String, client_secret: String, redirect_uri: String) -> Self {
        Self {
            sqlite: Arc::new(sqlite),
            client_id,
            client_secret,
            redirect_uri,
        }
    }

    pub async fn authenticate(
        State(google_auth): State<GoogleAuth>,
        Query(params): Query<HashMap<String, String>>,
    ) -> &'static str {
        if let Some(code) = params.get("code") {
            let mut form = HashMap::new();
            form.insert("code", code.as_str());
            form.insert("client_id", &google_auth.client_id);
            form.insert("client_secret", &google_auth.client_secret);
            form.insert("redirect_uri", &google_auth.redirect_uri);
            form.insert("grant_type", "authorization_code");

            let client = reqwest::Client::new();

            let response = client
                .post("https://oauth2.googleapis.com/token")
                .form(&form)
                .send()
                .await
                .unwrap();

            let status = response.status();
            let response_text = response.text().await.unwrap();

            if !status.is_success() {
                eprintln!("Token exchange failed with status {}: {}", status, response_text);
                return "Authentication failed. Please try again.";
            }

            let data: Token = match serde_json::from_str(&response_text) {
                Ok(token) => token,
                Err(e) => {
                    eprintln!("Failed to parse token response: {}. Response: {}", e, response_text);
                    // Try to parse as error response
                    if let Ok(error) = serde_json::from_str::<TokenError>(&response_text) {
                        eprintln!("Google OAuth error: {} - {:?}", error.error, error.error_description);
                    }
                    return "Authentication failed. Please try again.";
                }
            };

            let userinfo_response = client
                .get("https://www.googleapis.com/oauth2/v3/userinfo")
                .bearer_auth(&data.access_token)
                .send()
                .await;

            match userinfo_response {
                Ok(userinfo) => {
                    if !userinfo.status().is_success() {
                        eprintln!("Failed to get user info: {}", userinfo.status());
                        return "Failed to retrieve user information.";
                    }

                    match userinfo.json::<User>().await {
                        Ok(user) => {
                            google_auth.sqlite.upsert_user(user);
                            "Authentication successful!"
                        }
                        Err(e) => {
                            eprintln!("Failed to parse user info: {}", e);
                            "Failed to parse user information."
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to request user info: {}", e);
                    "Failed to retrieve user information."
                }
            }
        } else {
            "No authorization code provided."
        }
    }
}
