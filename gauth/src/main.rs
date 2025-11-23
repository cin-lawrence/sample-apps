use dioxus::prelude::*;

mod config;
mod db;
mod google;
mod models;
mod views;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[cfg(feature = "server")]
async fn google_auth_handler(
    axum::extract::State(google_auth): axum::extract::State<crate::google::GoogleAuth>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> &'static str {
    crate::google::GoogleAuth::authenticate(
        axum::extract::State(google_auth),
        axum::extract::Query(params),
    )
    .await
}

#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    use crate::config::Config;
    use crate::db::Sqlite;
    use crate::google::GoogleAuth;
    use dioxus::server::ServeConfig;

    let sqlite = Sqlite::new();
    sqlite.create_table();

    let config = Config::new();
    let google_auth = GoogleAuth::new(sqlite, config.client_id, config.client_secret);

    let address = dioxus::cli_config::fullstack_address_or_localhost();
    let router = axum::Router::new()
        .route("/google_auth", axum::routing::get(google_auth_handler))
        .with_state(google_auth)
        .serve_dioxus_application(ServeConfig::default(), App);

    let router = router.into_make_service();
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

#[cfg(not(feature = "server"))]
fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        div {
            "Home"
        }
    }
}
