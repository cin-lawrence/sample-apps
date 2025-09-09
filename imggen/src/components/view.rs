use dioxus::prelude::*;

use crate::models::image::ImageResponse;

#[derive(PartialEq, Clone, Props)]
pub struct ViewProps {
    image: Signal<ImageResponse>,
}

#[allow(non_snake_case)]
pub fn View(props: ViewProps) -> Element {
    let images = (props.image)();
    rsx! {
        for image in images.data.iter() {
            section { class: "is-flex",
                div { class: "container is-fluid",
                    div { class: "container has-text-centered",
                        div { class: "is-justify-content-center",
                            div { class: "level",
                                div { class: "level-item",
                                    figure { class: "image",
                                        img {
                                            alt: "",
                                            src: "{image.url}",
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
