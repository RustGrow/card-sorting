#![allow(non_snake_case)]
use crate::models::app_state::ApplicationData;
use crate::models::item::ItemCard;
use dioxus::prelude::*;

#[component]
pub fn ItemCardUi(card: ItemCard, id: usize, color: &'static str) -> Element {
    let mut data = use_context::<ApplicationData>();
    let mut dragStartState = data.currentCard;
    let mut dragOverBg = use_signal(|| false);
    let opacity = use_signal(|| false);

    rsx! {
        div {
            class: "w-1/6 h-1/3 rounded-lg border-4 border-solid m-5 flex justify-center items-center cursor-grab border-black text-white",
            class: if dragOverBg() { "bg-slate-500 bg-opacity-100" } else { color },
            class: if opacity() { "bg-opacity-100" } else { "bg-opacity-100" },
            draggable: card.draggable,
            ondragstart: move |_| {
                dragStartState.set(id);
                let dragStartStateID = dragStartState();
                let dataList = data.list.read();
                log::info!("DragStartID! {id:?} dragStartState {dragStartStateID:?}");
                log::info!("DataList! {dataList:?}");
            },
            ondragleave: move |_| {
                dragOverBg.set(false);
            },
            prevent_default: "ondragover",
            ondragover: move |_| {
                dragOverBg.set(true);
            },
            prevent_default: "ondrop",
            ondrop: move |_| {
                dragOverBg.set(false);
                if id != dragStartState() {
                    data.list.write().swap(id, dragStartState());
                }
                let dragStartStateID = dragStartState();
                log::info!("Swap DropCardID! {id:?} dragStartStateID {dragStartStateID:?}");
            },
            "{card.text} {card.id}"
        }
    }
}
