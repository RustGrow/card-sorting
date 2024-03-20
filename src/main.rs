#![allow(non_snake_case)]
mod app;
mod components;
mod models;
pub mod route;
use dioxus::prelude::*;
use log::LevelFilter;

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    launch(app::app);
}
