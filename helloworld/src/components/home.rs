use dioxus::prelude::*;

use crate::components::{echo::Echo, hero::Hero};

#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
        Echo {}
    }
}
