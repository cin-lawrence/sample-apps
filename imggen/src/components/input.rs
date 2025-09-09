use dioxus::prelude::*;

use crate::models::image::ImageResponse;
use crate::services::image::ImageService;

#[derive(PartialEq, Clone, Props)]
pub struct InputProps {
    api_token: Signal<String>,
    prompt: Signal<String>,
    num_image: Signal<String>,
    image: Signal<ImageResponse>,
    loading: Signal<String>,
}

#[allow(non_snake_case)]
pub fn Input(mut props: InputProps) -> Element {
    rsx! {
        div { class: "container",
            div { class: "columns",
                div { class: "column",
                    input { class: "input is-primary mt-4",
                        value: "{props.api_token}",
                        r#type: "text",
                        placeholder: "API Token",
                        oninput: move |evt| {
                            props.api_token.set(evt.value().clone());
                        },
                    }

                    input { class: "input is-primary mt-4",
                        value: "{props.prompt}",
                        r#type: "text",
                        placeholder: "MAX 1000 Digits",
                        oninput: move |evt| {
                            props.prompt.set(evt.value().clone());
                        },
                    }

                    input { class: "input is-primary mt-4",
                        value: "{props.num_image}",
                        r#type: "number",
                        min: "1",
                        max: "10",
                        oninput: move |evt| {
                            props.num_image.set(evt.value().clone());
                        },
                    }
                }
            }

            button { class: "button is-primary {props.loading}",
                onclick: move |_| {
                    spawn({
                        let mut loading = props.loading.clone();
                        loading.set("is-loading".to_string());
                        let mut image_request = props.image.clone();
                        let api_token_request = props.api_token.clone();
                        let prompt_request = props.prompt.clone();
                        let num_image_request = props.num_image.clone();
                        async move {
                            image_request.set(
                                ImageService::randomize_image(
                                    api_token_request(),
                                    prompt_request(),
                                    num_image_request(),
                                )
                                .await
                                .unwrap()
                            );
                            loading.set("".to_string());
                        }
                    });
                },
                "Generate image"
            }
            br {
            }
        }
    }
}
