use crate::models::image::ImageResponse;
use serde::Serialize;

pub struct ImageService {}

#[derive(Serialize)]
pub struct ImageRequest {
    prompt: String,
    n: i32,
    size: String,
}

impl ImageService {
    pub async fn randomize_image(
        api_token: String,
        prompt: String,
        num_image: String,
    ) -> reqwest::Result<ImageResponse> {
        let body = ImageRequest {
            prompt,
            n: num_image.parse::<i32>().unwrap_or(1),
            size: "1024x1024".to_string(),
        };

        let mut authorization = "Bearer ".to_string();
        authorization.push_str(&api_token);

        reqwest::Client::new()
            .post("https://api.openai.com/v1/images/generations")
            .json(&body)
            .header("Content-Type", "application/json")
            .header("Authorization", authorization)
            .send()
            .await?
            .json::<ImageResponse>()
            .await
    }
}
