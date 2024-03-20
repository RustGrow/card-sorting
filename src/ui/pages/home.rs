#![allow(non_snake_case)]
use crate::models::app_state::ApplicationData;
use crate::models::item::ItemCard;
use crate::ui::components::item_card_ui::ItemCardUi;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let mut data = use_context::<ApplicationData>();
    let colors = vec!["bg-red-500", "bg-blue-500", "bg-green-500", "bg-yellow-500"];
    let list: Signal<Vec<ItemCard>> = use_signal(|| {
        colors
            .iter()
            .enumerate()
            .map(|(id, color)| ItemCard::new(id, "Card", color))
            .collect()
    });
    *data.list.write() = list();
    // data.list.set(list());

    rsx! {
        div { class: "flex flex-row h-screen w-screen justify-center items-center",
            {
             data.list.read().iter().enumerate().map(|(id, card)| {
                rsx!{
                    ItemCardUi  {
                        card: *card,
                        id: id,
                        color: card.color,
                    }
                }

            })
        }
        }
    }
}
