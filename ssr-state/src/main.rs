use std::rc::Rc;

use axum_core::extract::{FromRef, FromRequest};
use dioxus::prelude::*;
use dioxus::fullstack::{FullstackContext, extract::State};
use reqwest::header::HeaderMap;

#[cfg(feature = "server")]
use {
    dioxus::fullstack::Lazy,
    dioxus::fullstack::axum,
    futures::lock::Mutex,
    sqlx::{Executor, Row},
    std::sync::LazyLock,
};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[cfg(feature = "server")]
static MESSAGES: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(Vec::new()));

#[post("/api/messages")]
async fn add_message() -> Result<()> {
    MESSAGES.lock().await.push("New message".to_string());
    Ok(())
}

#[get("/api/messages")]
async fn read_messages() -> Result<Vec<String>> {
    Ok(MESSAGES.lock().await.clone())
}

#[cfg(feature = "server")]
static DATABASE: Lazy<sqlx::SqlitePool> = Lazy::new(|| async move {
    use sqlx::sqlite::SqlitePoolOptions;
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with("sqlite::memory:?cache=shared".parse().unwrap())
        .await?;

    dioxus::Ok(pool)
});

#[get("/api/users")]
async fn get_users() -> Result<Vec<String>> {
    let pool = &*DATABASE;

    let users = pool
        .fetch_all(sqlx::query("SELECT name FROM users"))
        .await?
        .iter()
        .map(|row| row.get::<String, _>("name"))
        .collect::<Vec<_>>();

    Ok(users)
}

#[cfg(feature = "server")]
type BroadcastExtension = axum::Extension<tokio::sync::broadcast::Sender<String>>;

#[post("/api/broadcast", ext: BroadcastExtension)]
async fn broadcast_message() -> Result<usize> {
    let rt = Rc::new("asdasd".to_string());
    // send() returns Ok(count) if receivers are present, or Err(SendError) if no receivers
    match ext.send("New broadcast message".to_string()) {
        Ok(receiver_count) => {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            println!("rt: {}, receivers: {}", rt, receiver_count);
            Ok(receiver_count)
        }
        Err(_) => {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            println!("rt: {}, no receivers", rt);
            Err(ServerFnError::new("No receivers listening to broadcast channel").into())
        }
    }
}

#[derive(Clone)]
struct MyAppState {
    abc: i32,
}

impl FromRef<FullstackContext> for MyAppState {
    fn from_ref(state: &FullstackContext) -> Self {
        state.extension::<MyAppState>().unwrap()
    }
}

struct CustomExtractor {
    abc: i32,
    headermap: HeaderMap,
}

impl<S> FromRequest<S> for CustomExtractor
where
    MyAppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = ();

    async fn from_request(
        _req: axum_core::extract::Request,
        state: &S,
    ) -> std::result::Result<Self, Self::Rejection> {
        let state = MyAppState::from_ref(state);
        Ok(CustomExtractor {
            abc: state.abc,
            headermap: HeaderMap::new(),
        })
    }
}

#[post("/api/stateful", state: State<MyAppState>, ex: CustomExtractor)]
async fn app_state() -> Result<()> {
    println!("abc: {}", state.abc);
    println!("state abc: {}", ex.abc);
    println!("headermap: {:#?}", ex.headermap);
    Ok(())
}

fn main() {
    #[cfg(not(feature = "server"))]
    dioxus::launch(App);

    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        use dioxus::server::axum::Extension;

        // Create users table if it doesn't exist
        (&*DATABASE).execute(
            sqlx::query(
                "CREATE TABLE IF NOT EXISTS users (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL
                )"
            )
        )
        .await
        .expect("Failed to create users table");

        let router = dioxus::server::router(App)
            .layer(Extension(tokio::sync::broadcast::channel::<String>(16).0));

        let router = router.layer(Extension(MyAppState { abc: 42 }));

        Ok(router)
    });
}

#[component]
fn App() -> Element {
    let mut users = use_action(get_users);
    let mut messages = use_action(read_messages);
    let mut broadcast = use_action(broadcast_message);
    let mut add = use_action(add_message);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        div {
            button {
                onclick: move |_| users.call(),
                "Get Users"
            }
            pre {
                "{users.value():?}"
            }
            button {
                onclick: move |_| messages.call(),
                "Get Messages"
            }
            pre {
                "{messages.value():?}"
            }
            button {
                onclick: move |_| broadcast.call(),
                "Broadcast Message"
            }
            pre {
                "{broadcast.value():?}"
            }
            button {
                onclick: move |_| add.call(),
                "Add Message"
            }
            pre {
                "{add.value():?}"
            }
        }
    }
}
