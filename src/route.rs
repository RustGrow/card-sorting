use crate::ui::pages::error_page::Err404;
use crate::ui::pages::home::Home;
use dioxus::prelude::*;

#[derive(Routable, Clone, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/:..segments")]
    Err404 { segments: Vec<String> },
}
