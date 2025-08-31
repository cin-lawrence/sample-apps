use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
pub struct WeatherLocation {
    pub id: usize,
    pub name: String,
    pub latitude: f32,
    pub longitude: f32,
    pub country: String,
}

pub type WeatherLocations = Vec<WeatherLocation>;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SearchResponse {
    pub results: WeatherLocations,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
pub struct WeatherResponse {
    pub daily: DailyWeather,
    pub hourly: HourlyWeather,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
pub struct HourlyWeather {
    pub time: Vec<String>,
    pub temperature_2m: Vec<f32>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
pub struct DailyWeather {
    pub temperature_2m_min: Vec<f32>,
    pub temperature_2m_max: Vec<f32>,
}
