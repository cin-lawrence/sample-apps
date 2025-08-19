use dioxus::prelude::*;

#[server(EchoServer)]
pub async fn echo_server(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}
