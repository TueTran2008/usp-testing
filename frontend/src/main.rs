// use std::ops::Div;

use dioxus::{html::*, prelude::*};
// pub use elements::*;
// use dioxus_elements::div;

// const FAVICON: Asset = asset!("/assets/favicon.ico");
// const MAIN_CSS: Asset = asset!("/assets/main.css");
// const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx!(
        div {
            h1 {"HotDog!  🌭"}
        }
        div {
            img {
                src: "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg"
            }
        }
        div {
            button { id: "skip", "skip"}
            button { id: "save", "save!"}
        }
    )
}
