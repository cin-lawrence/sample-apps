use dioxus::prelude::*;
use tracing;

use crate::models::weather::{SearchResponse, WeatherLocation, WeatherLocations};
use crate::components::mapicon::MapIcon;

async fn get_locations(input: &str) -> reqwest::Result<WeatherLocations> {
    let res = reqwest::get(&format!(
        "https://geocoding-api.open-meteo.com/v1/search?name={input}"
    ))
    .await?
    .json::<SearchResponse>()
    .await?;

    tracing::info!("location response: {:?}", res.results);

    Ok(res.results)
}

#[allow(non_snake_case)]
#[component]
pub fn SearchBox(mut country: Signal<WeatherLocation>) -> Element {
    let mut input = use_signal(String::new);

    // let response = use_resource(move || {
    //     let query = input.read().clone();
    //     async move {
    //         get_locations(&query).await
    //     }
    // })
    // .suspend()?;

    rsx! {
        div {
            div { class: "inline-flex flex-col justify-center relative text-gray-500",
                div { class: "relative",
                    input {
                        class: "p-2 pl-8 rounded-lg border border-gray-200 bg-gray-200 focus:bg-white focus:outline-none focus:ring-2 focus:ring-yellow-600 focus:border-transparent",
                        placeholder: "Country name",
                        "type": "text",
                        autofocus: true,
                        value: "{input}",
                        oninput: move |event| {
                            input.set(event.value());
                        }
                    }
                    svg {
                        class: "w-4 h-4 absolute left-2.5 top-3.5",
                        "viewBox": "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        xmlns: "http://www.w3.org/2000/svg",
                        path {
                            d: "M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z",
                            "stroke-linejoin": "round",
                            "stroke-linecap": "round",
                            "stroke-width": "2"
                        }
                    }
                }
                //ul { class: "bg-white border border-gray-100 w-full mt-2 max-h-72 overflow-auto",
                //    if let Ok(locations) = &*response.read() {
                //        for location in locations.iter().cloned() {
                //            li { class: "pl-8 pr-2 py-1 border-b-2 border-gray-100 relative cursor-pointer hover:bg-yellow-50 hover:text-gray-900",
                //                //onclick: move |_| {
                //                //    println!("on click: {:?}", location);
                //                //    country.set(location.clone());
                //                //},
                //                MapIcon {}
                //                b {
                //                    "{location.name}"
                //                }
                //                " · {location.country}"
                //            }
                //        }
                //    }
                //}
            }
        }
    }
}
