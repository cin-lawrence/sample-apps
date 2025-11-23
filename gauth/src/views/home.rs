use crate::config::UIConfig;
use dioxus::prelude::*;

#[server]
async fn register(
    first_name: String,
    last_name: String,
    email: String,
    password: String,
) -> Result<String, ServerFnError> {
    use crate::db::Sqlite;

    let sqlite = Sqlite::new();
    let maybe_email = sqlite.check_user_email(&email);
    let message = match maybe_email {
        Some(_email) => "A user with this email already exists.",
        None => {
            use argon2::password_hash::{rand_core::OsRng, PasswordHasher, SaltString};
            use argon2::Argon2;

            let argon2 = Argon2::default();
            let salt = SaltString::generate(&mut OsRng);
            let password_hash = argon2
                .hash_password(password.as_bytes(), &salt)
                .unwrap()
                .to_string();

            sqlite.create_classic_user(first_name, last_name, email, password_hash);

            "User created."
        }
    };

    Ok(message.to_string())
}

#[server]
async fn login(email: String, password: String) -> Result<String, ServerFnError> {
    use crate::db::Sqlite;
    let sqlite = Sqlite::new();
    let maybe_user = sqlite.get_classic_user_by_email(&email);
    let message = match maybe_user {
        Some(user) => {
            use argon2::password_hash::{PasswordHash, PasswordVerifier};
            use argon2::Argon2;

            let argon2 = Argon2::default();
            let hash = PasswordHash::new(&user.3).unwrap();
            let result = argon2.verify_password(password.as_bytes(), &hash);

            if result.is_ok() {
                "Login successful"
            } else {
                "Invalid password"
            }
        }
        None => "Invalid login details",
    };

    Ok(message.to_string())
}

const GOOGLE_ICON: Asset = asset!("/assets/google-icon.svg");

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    let config = UIConfig::new();
    let mut toggle_form = use_signal(|| false);
    let mut button_text = use_signal(String::new);
    let mut response_msg = use_signal(String::new);

    let mut first_name = use_signal(String::new);
    let mut last_name = use_signal(String::new);
    let mut email = use_signal(String::new);
    let password = use_signal(String::new);

    if toggle_form() == false {
        button_text.set(String::from("Create Account"));
    } else {
        button_text.set(String::from("Sign In"));
    }

    rsx! {
        div { class: "login-container",
            if toggle_form() == false {
                div {
                    h1 { "Create an account" },
                    div { class: "row sub-text-container",
                        p { "Already have an account?" },
                        a {
                            onclick: move |_| {
                                toggle_form.set(true)
                            },
                            "Login"
                        }
                    }
                }
            } else {
                div {
                    h1 { "Sign In" },
                    div { class: "row sub-text-container",
                        p { "Don't have an account?" },
                        a {
                            onclick: move |_| {
                                toggle_form.set(false)
                            },
                            "Sign Up"
                        }
                    }
                }
            }

            div { class: "msg",
                "{response_msg}"
            }

            if toggle_form() == false {
                div { class: "row col2",
                    div {
                        input {
                            type: "text",
                            placeholder: "First Name",
                            oninput: move |event| {
                                first_name.set(event.value());
                            },
                        }
                    }
                    div {
                        input {
                            type: "text",
                            placeholder: "Last Name",
                            oninput: move |event| {
                                last_name.set(event.value());
                            },
                        }
                    }
                },
            }

            div {
                class: "row",
                input {
                    type: "text",
                    placeholder: "Email",
                    oninput: move |event| {
                        email.set(event.value());
                    },
                }
            },
            div {
                button {
                    class: "btn-primary",
                    onclick: move |_| async move {
                        if toggle_form() == false {
                            let response_text = register(
                                first_name(),
                                last_name(),
                                email(),
                                password(),
                            )
                                .await
                                .unwrap();
                            response_msg.set(response_text);
                        } else {
                            let response_text = login(email(), password()).await.unwrap();
                            response_msg.set(response_text);
                        }
                    },
                    {button_text()}
                }
            },
            fieldset {
                legend { "Or signin with" }
                div {
                    class: "row",
                    a {
                        class: "btn-signin-google",
                        href : format!("https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type=code&scope=email profile", config.client_id, config.redirect_url),
                        img {
                            src: GOOGLE_ICON,
                        },
                        "Sign in with Google"
                    }
                }
            }
        }
    }
}
