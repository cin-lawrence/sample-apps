use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut count = use_signal(|| 0);
    let mut message = use_action(get_greeting);

    rsx! {
        div {
            style: "padding: 2rem; font-family: Arial, san-serif;",
            h1 {
                "Hello, Dioxus Fullstack!"
            }
            div {
                style: "margin: 1rem 0;",
                h2 {
                    "Client Counter: {count}"
                }
                button {
                    onclick: move |_| count += 1,
                    "Increment"
                }
                button {
                    onclick: move |_| count -= 1,
                    "Decrement"
                }
            }
            div {
                style: "margin: 1rem 0;",
                h2 {
                    "Server Greeting"
                }
                button {
                    onclick: move |_| message.call("World".to_string(), 30),
                    "Get Server Greeting"
                }
                if message.pending() {
                    p { "Loading..." }
                }
                p { "{message:#?}" }
            }
        }
    }
}

#[post("/api/greeting/{name}/{age}")]
async fn get_greeting(name: String, age: i32) -> Result<String> {
    Ok(format!(
            "Hello from the server, {}! You are {} years old.",
            name,
            age,
    ))
}
