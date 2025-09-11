use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::models::User;
#[cfg(feature = "server")]
use crate::repos::database::Session;

/// The Blog page component that will be rendered when the current route is `[Route::Blog]`
///
/// The component takes a `id` prop of type `i32` from the route enum. Whenever the id changes, the component function will be
/// re-run and the rendered HTML will be updated.
#[component]
pub fn Blog() -> Element {
    let mut user_name = use_signal(|| "?".to_string());
    let mut permissions = use_signal(|| "?".to_string());

    rsx! {
        div {
            button { onclick: move |_| {
                    async move {
                        login().await.unwrap();
                    }
                },
                "Login Test User"
            }
        }
        div {
            button {
                onclick: move |_| async move {
                    if let Ok(data) = get_user_name().await {
                        user_name.set(data);
                    }
                },
                "Get User Name"
            }
            "User name: {user_name}"
        }
        div {
            button {
                onclick: move |_| async move {
                    if let Ok(data) = get_permissions().await {
                        permissions.set(data);
                    }
                },
                "Get Permissions"
            }
            "Permissions: {permissions}"
        }
    }
}

#[server(GetUserName)]
pub async fn get_user_name() -> Result<String, ServerFnError> {
    let session: Session = extract().await?;
    Ok(session.0.current_user.clone().unwrap().username.to_string())
}

#[server(Login)]
pub async fn login() -> Result<(), ServerFnError> {
    let auth: Session = extract().await?;
    auth.login_user(2);
    Ok(())
}

#[server(Permissions)]
pub async fn get_permissions() -> Result<String, ServerFnError> {
    let method: axum::http::Method = extract().await?;
    let auth: Session = extract().await?;
    let current_user = auth.current_user.clone().unwrap_or_default();

    // lets check permissions only and not worry about if they are anon or not
    if !axum_session_auth::Auth::<User, i64, sqlx::SqlitePool>::build(
        [axum::http::Method::POST],
        false,
    )
    .requires(axum_session_auth::Rights::any([
        axum_session_auth::Rights::permission("Category::View"),
        axum_session_auth::Rights::permission("Admin::View"),
    ]))
    .validate(&current_user, &method, None)
    .await
    {
        return Ok(format!(
            "User {}, Does not have permissions needed to view this page please login",
            current_user.username
        ));
    }

    Ok(format!(
        "User {:?} has Permissions needed. Here are the Users permissions: {:?}",
        current_user.username, current_user.permissions
    ))
}
