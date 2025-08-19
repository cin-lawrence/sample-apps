use dioxus::prelude::*;

use crate::models::weather::WeatherResponse;

#[allow(non_snake_case)]
#[component]
pub fn Forecast(weather: WeatherResponse) -> Element {
    let today = (weather.daily.temperature_2m_max.first().unwrap()
        + weather.daily.temperature_2m_max.first().unwrap())
        / 2.0;
    let tomorrow = (weather.daily.temperature_2m_max.get(1).unwrap()
        + weather.daily.temperature_2m_max.get(1).unwrap())
        / 2.0;
    let past_tomorrow = (weather.daily.temperature_2m_max.get(2).unwrap()
        + weather.daily.temperature_2m_max.get(2).unwrap())
        / 2.0;
    rsx!(
        div { class: "px-6 pt-4 relative",
            div { class: "w-full h-px bg-gray-100 mb-4" }
            div { p { class: "text-center w-full mb-4", "👇 Forecast 📆" } }
            div { class: "text-center justify-between items-center flex",
                div { class: "text-center mb-0 flex items-center justify-center flex-col mx-4 w-16",
                    span { class: "block my-1", "Today" }
                    span { class: "block my-1", "{today}°" }
                }
                div { class: "text-center mb-0 flex items-center justify-center flex-col mx-8 w-16",
                    span { class: "block my-1", "Tomorrow" }
                    span { class: "block my-1", "{tomorrow}°" }
                }
                div { class: "text-center mb-0 flex items-center justify-center flex-col mx-2 w-30",
                    span { class: "block my-1", "Past Tomorrow" }
                    span { class: "block my-1", "{past_tomorrow}°" }
                }
            }
        }
    )
}
