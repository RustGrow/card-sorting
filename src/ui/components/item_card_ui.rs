#![allow(non_snake_case)]
use crate::models::app_state::ApplicationData;
use crate::models::item::ItemCard;
use dioxus::prelude::*;
use dioxus_signals::*;

#[component]
pub fn ItemCardUi(cx: Scope, card: ItemCard, id: usize, color: &'static str) -> Element {
    let data = ApplicationData::use_app_data(cx);
    let list = data.list;
    let dragStartState = data.currentCard;
    let dragOverBg = use_signal(cx, || false);

    render! {
        div {
            class: "w-1/6 h-1/3 rounded-lg border-4 border-solid m-5 flex justify-center items-center cursor-grab border-black text-white",
            class: if *dragOverBg.read() { "bg-slate-500" } else { color },
            draggable: card.draggable,
            ondragstart: move |_| {
                *dragStartState.write() = *id;
                let cc = *dragStartState.read();
                log::info!("DragStartID! {id:?} dragStartState {cc:?}");
            },
            ondragleave: move |_| {
                *dragOverBg.write() = false;
            },
            prevent_default: "ondragover",
            ondragover: move |_| {
                *dragOverBg.write() = true;
            },
            prevent_default: "ondrop",
            ondrop: move |_| {
                *dragOverBg.write() = false;
                list.write().swap(*id, *dragStartState.read());
                let cc = *dragStartState.read();
                log::info!("DropCardID! {id:?} dragStartState {cc:?}");
            },
            "{card.text} {card.id}"
        }
    }
}
