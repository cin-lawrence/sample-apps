use crate::models::weather::{SearchResponse, WeatherLocation, WeatherLocations, WeatherResponse};

pub struct WeatherService {}

const WEATHER_API: &str = "https://api.open-meteo.com/v1/forecast?latitude={lat}&longitude={lon}&hourly=temperature_2m&daily=temperature_2m_max,temperature_2m_min,apparent_temperature_max,apparent_temperature_min&timezone=GMT";

impl WeatherService {
    pub async fn get_weather(location: &WeatherLocation) -> reqwest::Result<WeatherResponse> {
        let url = WEATHER_API
            .replace("{lat}", &location.latitude.to_string())
            .replace("{lon}", &location.longitude.to_string());

        reqwest::Client::new()
            .get(&url)
            .send()
            .await?
            .json::<WeatherResponse>()
            .await
    }

    pub async fn get_locations(input: &str) -> reqwest::Result<WeatherLocations> {
        let res = reqwest::get(&format!(
            "https://geocoding-api.open-meteo.com/v1/search?name={input}"
        ))
        .await?
        .json::<SearchResponse>()
        .await?;

        Ok(res.results)
    }
}
