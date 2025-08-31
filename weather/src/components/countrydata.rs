use chrono::Utc;
use dioxus::prelude::*;

use crate::models::weather::{WeatherLocation, WeatherResponse};

#[derive(PartialEq, Clone, Props)]
pub struct CountryDataProps {
    weather: WeatherResponse,
    country: WeatherLocation,
}


#[allow(non_snake_case)]
pub fn CountryData(props: CountryDataProps) -> Element {
    let today = Utc::now().format("%y/%m/%d");
    let max_temp = props.weather.daily.temperature_2m_max.first().unwrap();
    let min_temp = props.weather.daily.temperature_2m_min.first().unwrap();

    rsx! {
        div { class: "flex mb-4 justify-between items-center",
            div {
                h5 { class: "mb-0 font-medium text-xl", "{props.country.name} 🏞️" }
                h6 { class: "mb-0", "{today}" }
            }
            div {
                div { class: "flex items-center",
                    span { "Temp min" }
                    span { class: "px-2 inline-block", "👉 {min_temp}°" }
                }
                div { class: "flex items-center",
                    span { "Temp max" }
                    span { class: "px-2 inline-block ", "👉 {max_temp}º" }
                }
            }
        }
    }
}
