use axum_core::response::IntoResponse;
use dioxus::prelude::*;
use dioxus_fullstack::FromResponse;
use dioxus_fullstack::http::StatusCode;
use serde::{Deserialize, Serialize};

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut echo_action = use_action(echo);
    let mut chat_action = use_action(chat);
    let mut dog_data = use_action(get_data);
    let mut custom_data = use_action(get_custom_data);
    let mut anonymous_action = use_action(anonymous);
    let mut custom_anonymous_action = use_action(custom_anonymous);
    let mut custom_response_action = use_action(get_custom_response);

    rsx! {
        h1 {
            "Server Functions Example"
        }
        div {
            display: "flex",
            flex_direction: "column",
            gap: "8px",

            button { onclick: move |_| echo_action.call("Hello from client".into()), "Echo: Hello" }
            button { onclick: move |_| chat_action.call(42u32, Some(7u32)), "Chat (user 42, room 7)" }
            button { onclick: move |_| dog_data.call(), "Get dog data" }
            button { onclick: move |_| custom_data.call(), "Get custom data" }
            button { onclick: move |_| anonymous_action.call(), "Call anonymous" }
            button { onclick: move |_| custom_anonymous_action.call(), "Call custom anonymous" }
            button { onclick: move |_| custom_response_action.call(), "Get custom response" }

            button {
                onclick: move |_| {
                    echo_action.reset();
                    chat_action.reset();
                    dog_data.reset();
                    custom_data.reset();
                    anonymous_action.reset();
                    custom_anonymous_action.reset();
                    custom_response_action.reset();
                },
                "Clear results"
            }

            pre { "Echo result: {echo_action.value():#?}" }
            pre { "Chat result: {chat_action.value():#?}" }
            pre { "Dog data: {dog_data.value():#?}" }
            pre { "Custom data: {custom_data.value():#?}" }
            pre { "Anonymous: {anonymous_action.value():#?}" }
            pre { "Custom anonymous: {custom_anonymous_action.value():#?}" }
            pre { "Custom response: {custom_response_action.value():#?}" }
        }
    }
}

#[post("/api/echo")]
async fn echo(body: String) -> Result<String> {
    Ok(body)
}

#[post("/api/{user_id}/chat?room_id", headers: dioxus_fullstack::HeaderMap)]
async fn chat(user_id: u32, room_id: Option<u32>) -> Result<String> {
    Ok(format!(
            "user ID: {}, Room ID: {} - Headers: {:#?}",
            user_id,
            room_id.map_or("None".to_string(), |id| id.to_string()),
            headers,
    ))
}

#[derive(Serialize, Deserialize, Debug)]
struct DogData {
    name: String,
    age: u8,
}

#[get("/api/dog")]
async fn get_data() -> Result<DogData> {
    Ok(DogData {
        name: "Fido".to_string(),
        age: 4,
    })
}

#[derive(Debug)]
struct CustomData {
    message: String,
}

impl IntoResponse for CustomData {
    fn into_response(self) -> axum_core::response::Response {
        axum_core::response::Response::builder()
            .status(StatusCode::ACCEPTED)
            .body(serde_json::to_string(&self.message).unwrap().into())
            .unwrap()
    }
}

impl FromResponse for CustomData {
    async fn from_response(res: dioxus_fullstack::ClientResponse) -> Result<Self, ServerFnError> {
        let message = res.json::<String>().await?;
        Ok(CustomData { message })
    }
}

#[get("/api/custom")]
async fn get_custom_data() -> Result<CustomData> {
    Ok(CustomData {
        message: "Hello from the server!".to_string(),
    })
}

#[get("/api/custom_response")]
async fn get_custom_response() -> Result<axum_core::response::Response> {
    Ok(axum_core::response::Response::builder()
        .status(StatusCode::CREATED)
        .body("Created!".to_string())
        .unwrap()
        .into_response()
    )
}

#[server]
async fn anonymous() -> Result<String> {
    Ok("Hello from an anonymous server function!".to_string())
}

#[server(prefix = "/api/custom", endpoint = "my_anonymous")]
async fn custom_anonymous() -> Result<String> {
    Ok("Hello from a custom anonymous server function!".to_string())
}
