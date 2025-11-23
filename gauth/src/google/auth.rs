use axum::extract::{Query, State};
use serde::Deserialize;

use crate::models::User;
use crate::db::Sqlite;

use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct Token {
    access_token: String,
}

#[derive(Clone)]
pub struct GoogleAuth {
    pub sqlite: Arc<Sqlite>,
    pub client_id: String,
    pub client_secret: String,
}

impl GoogleAuth {
    pub fn new(sqlite: Sqlite, client_id: String, client_secret: String) -> Self {
        Self {
            sqlite: Arc::new(sqlite),
            client_id,
            client_secret,
        }
    }

    pub async fn authenticate(
        State(google_auth): State<GoogleAuth>,
        Query(params): Query<HashMap<String, String>>,
    ) -> &'static str {
        if let Some(code) = params.get("code") {
            let mut form = HashMap::new();
            form.insert("code", code.as_str());
            form.insert("client_id",  &google_auth.client_id);
            form.insert("client_secret", &google_auth.client_secret);
            form.insert("grant_type", "authorization_code");

            let client = reqwest::Client::new();

            let response = client
                .post("https://oauth2.googleapis.com/token")
                .form(&form)
                .send()
                .await
                .unwrap();

            let data: Token = response.json::<Token>().await.unwrap();
            let userinfo = client
                .get("https://www.googleapis.com/oauth2/v3/userinfo")
                .bearer_auth(data.access_token)
                .send()
                .await
                .unwrap();

            let user: User = userinfo.json::<User>().await.unwrap();
            google_auth.sqlite.upsert_user(user);
        }

        "Process completed"
    }
}

