use dioxus::prelude::*;

use crate::components::input::Input;
use crate::components::view::View;
use crate::models::image::ImageResponse;

#[allow(non_snake_case)]
pub fn Image() -> Element {
    let api_token = use_signal(|| "".to_string());
    let prompt = use_signal(|| "".to_string());
    let num_image = use_signal(|| 1.to_string());
    let image = use_signal(|| ImageResponse {
        created: 0,
        data: Vec::new(),
    });
    let loading = use_signal(|| "".to_string());

    rsx! {
        Input {
            api_token,
            prompt,
            num_image,
            image,
            loading,
        }
        View {
            image,
        }
    }
}
