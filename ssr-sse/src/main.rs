use dioxus::prelude::*;
use dioxus_fullstack::ServerEvents;
use serde::{Deserialize, Serialize};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut events = use_signal(Vec::new);

    use_future(move || async move {
        let mut stream = listen_for_chages().await?;

        while let Some(Ok(event)) = stream.recv().await {
            events.push(event);
        }

        dioxus::Ok(())
    });
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        h1 {
            "Events from server: "
        }
        for msg in events.read().iter().rev() {
            pre { "{msg:?}" }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum MyServerEvent {
    Yay { message: String },
    Nay { error: String },
}

#[get("/api/sse")]
async fn listen_for_chages() -> Result<ServerEvents<MyServerEvent>> {
    use std::time::Duration;

    Ok(ServerEvents::new(|mut tx| async move {
        let mut count = 1;

        loop {
            let msg = if count % 5 == 0 {
                MyServerEvent::Nay {
                    error: "An error occurred".into(),
                }
            } else {
                MyServerEvent::Yay {
                    message: format!("Hello number {count}"),
                }
            };

            if tx.send(msg).await.is_err() {
                break;
            }

            count += 1;

            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }))
}
