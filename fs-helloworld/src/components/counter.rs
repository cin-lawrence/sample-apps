use dioxus::prelude::*;

use crate::services::{get_server_data, post_server_data};

#[allow(non_snake_case)]
pub fn Counter() -> Element {
    let mut count = use_signal(|| 0);
    let mut text = use_signal(|| "...".to_string());
    // let server_future = use_server_future(get_server_data);

    rsx! {
        h1 { "High-Five counter: {count}" }
        button {
            onclick: move |_| count += 1,
            "Up high!"
        }
        button {
            onclick: move |_| count -= 1,
            "Down low!"
        }
        button {
            onclick: move |_| async move {
                match get_server_data().await {
                    Ok(data) => {
                        tracing::info!("Client received: {}", data);
                        text.set(data.clone());
                        post_server_data(data).await.unwrap();
                    },
                    Err(_) => {},
                }
            },
            "Run a server function!"
            }
        "Server said: {text}"
    }
}
