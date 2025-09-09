use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Props, Clone)]
pub struct UrlImage {
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Props, Clone)]
pub struct ImageResponse {
    pub created: i32,
    pub data: Vec<UrlImage>,
}
