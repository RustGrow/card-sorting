#![allow(non_snake_case)]
use super::item_card_ui::ItemCardUi;
use crate::models::app_state::ApplicationData;
use crate::models::item::ItemCard;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let mut data = use_context::<ApplicationData>();
    use_hook(|| {
        let colors = vec!["bg-red-500", "bg-blue-500", "bg-green-500", "bg-yellow-500"];
        *data.list.write() = colors
            .iter()
            .enumerate()
            .map(|(id, color)| ItemCard::new(id, "Card", color))
            .collect();
    });

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
