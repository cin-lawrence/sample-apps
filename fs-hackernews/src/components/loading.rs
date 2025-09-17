use dioxus::prelude::*;


#[allow(non_snake_case)]
pub fn LoadingIndicator() -> Element {
    rsx! {
        div { class: "spinner",
        }
    }
}
