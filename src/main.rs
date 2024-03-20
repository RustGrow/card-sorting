#![allow(non_snake_case)]
mod app;
mod models;
pub mod route;
mod ui;
use dioxus::prelude::*;
use log::LevelFilter;

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    launch(app::app);
}
