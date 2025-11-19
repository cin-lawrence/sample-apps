use dioxus::prelude::*;


#[allow(non_snake_case)]
pub fn ErrorPage() -> Element {
    rsx! {
        section { class: "py-20",
            div { class: "container mx-auto px-4",
                div { class: "flex flex-wrap -mx-4 mb-24 text-center",
                    "An internal error has occurred"
                }
            }
        }
    }
}
