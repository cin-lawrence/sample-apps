use crate::models::{Cart, Product, User};
use crate::enums::Sort;

pub struct FakeStoreAPI {}

const FAKE_STORE_API: &str = "https://fakestoreapi.com";

impl FakeStoreAPI {
    pub async fn fetch_user_carts(user_id: usize) -> reqwest::Result<Vec<Cart>> {
        reqwest::get(
            format!("{FAKE_STORE_API}/carts/user/{user_id}?startdate=2019-12-10&enddate=2023-01-01")
        )
            .await?
            .json::<Vec<Cart>>()
            .await
    }

    pub async fn fetch_user(user_id: usize) -> reqwest::Result<User> {
        reqwest::get(
            format!("{FAKE_STORE_API}/users/{user_id}")
        )
            .await?
            .json::<User>()
            .await
    }

    pub async fn fetch_product(product_id: usize) -> reqwest::Result<Product> {
        reqwest::get(
            format!("{FAKE_STORE_API}/products/{product_id}")
        )
            .await?
            .json::<Product>()
            .await
    }

    pub async fn fetch_products(count: usize, sort: Sort) -> reqwest::Result<Vec<Product>> {
        reqwest::get(
            format!("{FAKE_STORE_API}/products/?sort={sort}&limit={count}")
        )
            .await?
            .json()
            .await
    }
}
