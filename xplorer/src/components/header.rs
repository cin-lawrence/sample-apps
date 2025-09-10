use dioxus::prelude::*;

use crate::models::file::Files;

#[derive(PartialEq, Clone, Props)]
pub struct HeaderProps {
    pub files: Signal<Files>,
}

#[allow(non_snake_case)]
pub fn Header(props: HeaderProps) -> Element {
    let mut files = props.files;

    rsx! {
        header {
            i { class: "material-icons icon-menu", "menu" }
            h1 { "Files: {(*files.read()).current()}" }
            span { }
            i { class: "material-icons",
                onclick: move |_| files.write().go_up(),
                "logout"
            }
        }
    }
}
