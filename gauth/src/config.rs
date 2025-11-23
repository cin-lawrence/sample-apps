use dotenv::dotenv;
use std::env;

pub const GOOGLE_REDIRECT_URL: &str = "http://127.0.0.1:8888/google_auth";

pub struct UIConfig {
    pub client_id: String,
    pub redirect_url: String,
}

impl UIConfig {
    pub fn new() -> Self {
        dotenv().ok();

        Self {
            client_id: env::var("CLIENT_ID").unwrap().to_string(),
            redirect_url: env::var("GOOGLE_REDIRECT_URL")
                .unwrap_or_else(|_| GOOGLE_REDIRECT_URL.to_string()),
        }
    }
}

#[cfg(feature = "server")]
pub struct Config {
    pub client_id: String,
    pub client_secret: String,
}

#[cfg(feature = "server")]
impl Config {
    pub fn new() -> Self {
        dotenv().ok();

        Self {
            client_id: env::var("CLIENT_ID")
                .expect("CLIENT_ID environment variable is required. Please create a .env file with CLIENT_ID=your_google_client_id"),
            client_secret: env::var("CLIENT_SECRET")
                .expect("CLIENT_SECRET environment variable is required. Please create a .env file with CLIENT_SECRET=your_google_client_secret"),
        }
    }
}
