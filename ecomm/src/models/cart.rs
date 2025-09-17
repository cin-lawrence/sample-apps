use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Clone)]
pub struct ProductInCart {
    #[serde(rename = "productId")]
    product_id: usize,
    quantity: usize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Cart {
    id: usize,
    #[serde(rename = "userId")]
    user_id: usize,
    data: String,
    products: Vec<ProductInCart>,
    date: DateTime<Utc>,
}
