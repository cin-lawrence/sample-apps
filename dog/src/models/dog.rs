use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct AllBreeds {
    pub message: HashMap<String, Vec<String>>,
    pub status: String,
}

#[derive(Serialize, Deserialize)]
pub struct RandomImageByBreed {
    pub message: String,
    pub status: String,
}

pub struct DogAPI {}

impl DogAPI {
    pub async fn list_all_breeds() -> reqwest::Result<AllBreeds> {
        reqwest::Client::new()
            .get("https://dog.ceo/api/breeds/list/all")
            .send()
            .await?
            .json::<AllBreeds>()
            .await
    }

    pub async fn random_image_by_breed(breed: &str) -> reqwest::Result<RandomImageByBreed> {
        reqwest::Client::new()
        .get(format!("https://dog.ceo/api/breed/{breed}/images/random"))
        .send()
        .await?
        .json::<RandomImageByBreed>()
        .await
    }
}
