use dioxus::prelude::*;

use crate::components::{hero::Hero, echo::Echo};

#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
        Echo {}
    }
}
