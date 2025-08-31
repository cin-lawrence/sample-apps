use dioxus::prelude::*;

use crate::components::countrydata::CountryData;
use crate::components::forecast::Forecast;
use crate::components::searchbox::SearchBox;
use crate::models::weather::WeatherLocation;
use crate::services::weather::WeatherService;

#[allow(non_snake_case)]
pub fn Weather() -> Element {
    let country = use_signal(|| WeatherLocation {
        name: "Berlin".to_string(),
        country: "Germany".to_string(),
        latitude: 52.5244,
        longitude: 13.4105,
        id: 2950159,
    });

    let response = use_resource(move || async move {
            WeatherService::get_weather(&country()).await
    })
    .suspend()?;

    rsx! {
        link {
            rel: "stylesheet",
            href: "https://unpkg.com/tailwindcss@^2.0/dist/tailwind.min.css"
        }
        div { class: "mx-auto p-4 bg-gray-100 h-screen flex justify-center",
            div { class: "flex items-center justify-center flex-row",
                div { class: "flex items-start justify-center flex-row",
                    SearchBox { country: country }
                    div { class: "flex flex-wrap w-full px-2",
                        div { class: "bg-gray-900 text-white relative min-w-0 break-words rounded-lg overflow-hidden shadow-sm mb-4 w-full bg-white dark:bg-gray-600",
                            div { class: "px-6 py-6 relative",
                                match &*response.read() {
                                    Ok(weather) => rsx! {
                                        CountryData {
                                            country: country(),
                                            weather: weather.clone(),
                                        }
                                        Forecast {
                                            weather: weather.clone(),
                                        }
                                    },
                                    Err(err) => rsx! {
                                        "failed to fetch response: {err}"
                                    },
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
