#![allow(non_snake_case)]
mod components;
mod models;
pub mod route;
use crate::models::app_state::ApplicationData;
use crate::route::Route;
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

pub const STYLE: &str = asset!("./assets/tailwind.css");
fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    dioxus::launch(App);
}

pub fn App() -> Element {
    use_context_provider(ApplicationData::new);
    rsx! {
        document::Link { rel: "stylesheet", href: STYLE }
        Router::<Route> {}
    }
}
