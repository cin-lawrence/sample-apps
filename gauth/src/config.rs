use std::env;
use dotenv::dotenv;

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
            redirect_url: env::var("GOOGLE_REDIRECT_URL").unwrap_or_else(
                |_| GOOGLE_REDIRECT_URL.to_string(),
            ),
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
            client_id: env::var("CLIENT_ID").unwrap().to_string(),
            client_secret: env::var("CLIENT_SECRET").unwrap().to_string(),
        }
    }
}
