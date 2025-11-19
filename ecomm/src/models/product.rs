use super::Rating;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Default)]
pub struct Product {
    pub id: u32,
    pub title: String,
    pub price: f32,
    pub description: String,
    pub category: String,
    pub image: String,
    pub rating: Rating,
}
