use dioxus::prelude::*;
use std::path::Path;
use crate::services::login::LoginSession;

#[component]
pub fn Login() -> Element {
    let mut username = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());
    let login_status = use_signal(|| "Not logged in".to_string());

    rsx! {
        div {
            h1 { "Login Example" }
            div {
                input {
                    r#type: "text",
                    placeholder: "Username",
                    value: username.read().clone(),
                    oninput: move |e| username.set(e.value().clone()),
                }
            }
            div {
                input {
                    r#type: "password",
                    placeholder: "Password",
                    value: password.read().clone(),
                    oninput: move |e| password.set(e.value().clone()),
                }
            }
            button {
                onclick: move |_| {
                    let username = username.read().clone();
                    let password = password.read().clone();
                    let mut login_status = login_status.clone();
                    async move {
                        match LoginSession::login(&username, &password, "1").await {
                            Ok(session) => {
                                login_status.set("Login successful!".to_string());
                                session.save_cookies(Path::new("cookies.txt")).unwrap();
                            },
                            Err(e) => {
                                login_status.set(format!("Login failed: {}", e));
                            }
                        }
                    }
                },
                "Login"
            }
            div {
                p { "{login_status}" }
            }
        }
    }
}