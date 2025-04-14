use dioxus::{html::*, prelude::*};

static CSS: Asset = asset!("/assets/main.css");
fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx!(
        document::Stylesheet { href: CSS }
        div{
            class: "login-container",
            h1 {"Login"},
            input {
                class: "input-field",
                r#type: "text",
                placeholder: "Username",
                oninput: move |e| println!("Username {}", e.value())
            }
            input {
                class: "input-field",
                r#type: "password",
                placeholder: "Password",
                oninput: move |e| println!("Password {}", e.value())
            }
            button {
                class: "login-button",
                onclick: move |_| println!("Login clicked"),
                "Login"
            }
        }
    )
}
