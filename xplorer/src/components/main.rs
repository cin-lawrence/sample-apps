use dioxus::prelude::*;

use crate::components::header::Header;
use crate::models::file::Files;

#[allow(non_snake_case)]
pub fn Main() -> Element {
    let mut files = use_signal(|| Files::new());

    rsx! {
        Header {
            files,
        }

        main {
            for (dir_id, path) in (*files.read()).path_names.iter().enumerate() {
                div { class: "folder",
                    key: "{path.name}",
                    i { class: "material-icons",
                        onclick: move |_| files.write().enter_dir(dir_id),
                        if path.is_directory { "folder" } else { "description" }
                        p { class: "cooltip",
                            "0 folders / 0 files"
                        }
                    }
                    h1 { "{path.name}" }
                }
            }

            if let Some(err) = files.read().err.as_ref() {
                div {
                    code { "{err}" }
                    button { onclick: move |_| files.write().clear_err(), "x" }
                }
            }
        }
    }
}
