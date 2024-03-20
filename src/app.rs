#![allow(non_snake_case)]
use crate::models::app_state::ApplicationData;
use crate::route::Route;
use dioxus::prelude::*;

pub fn app() -> Element {
    use_context_provider(ApplicationData::new);
    rsx! { Router::<Route> {} }
}
