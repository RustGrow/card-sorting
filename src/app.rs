#![allow(non_snake_case)]
use crate::models::app_state::ApplicationData;
use crate::models::item::ItemCard;
use crate::route::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use dioxus_signals::*;

pub fn App(cx: Scope) -> Element {
    let colors = vec!["bg-red-500", "bg-blue-500", "bg-green-500", "bg-yellow-500"];
    let list: Signal<Vec<ItemCard>> = use_signal(cx, || {
        colors
            .iter()
            .enumerate()
            .map(|(id, color)| ItemCard::new(id, "Card", color))
            .collect()
    });
    let currentCard = use_signal(cx, || 0 as usize);

    use_context_provider(cx, || ApplicationData::new(list, currentCard));

    render! { Router::<Route> {} }
}
