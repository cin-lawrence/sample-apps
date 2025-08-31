use dioxus::prelude::*;

use crate::components::card::Card;
use crate::models::dog::DogAPI;

#[component]
pub fn Dog() -> Element {
    let mut selected_breed = use_signal(String::new);
    let mut search_input = use_signal(String::new);

    let response = use_resource(move || async move {
        DogAPI::list_all_breeds().await
    })
    .suspend()?;


    rsx! {
        div {
            nav { class: "bg-white shadow p-8 border-b border-gray-400 sticky top-0",
                div { class: "flex justify-center align-vertical",
                    input {
                        class: "p-4 w-full",
                        "type": "text",
                        value: "{search_input}",
                        placeholder: "Search for doggo",
                        oninput: move |evt| search_input.set(evt.value()),
                        onkeydown: move |evt| { if evt.key() == Key::Enter {} }
                    }
                }
            }
            div { class: "px-2 flex gap-2",
                div { class: "grow w-full",
                    match &*response.read() {
                        Ok(breeds) => {
                            let current_search = search_input();
                            rsx! {
                                for (breed, subbreeds) in breeds.message.clone() {
                                    if current_search.is_empty() || breed.contains(&current_search) {
                                        Card {
                                            key: "{breed}",
                                            title: breed.to_string(),
                                            list: subbreeds.clone(),
                                            onclick: move |_| {
                                                let breed = breed.to_string();
                                                spawn(async move {
                                                    if let Ok(image) = DogAPI::random_image_by_breed(&breed).await {
                                                        selected_breed.set(image.message);
                                                    }
                                                });
                                            },
                                        }
                                    } else {
                                        {}
                                    }
                                }
                            }
                        },
                        Err(err) => rsx! {
                            "Failed to fetch response: {err}"
                        }
                    }
                }
                if !selected_breed().is_empty() {
                    div { class: "fixed bottom-0 right-0 m-10 shadow-ig",
                        img {
                            class: "w-80 h-80 rounded-ig",
                            src: "{selected_breed}"
                        }
                    }
                }
            }
        }
    }
}
