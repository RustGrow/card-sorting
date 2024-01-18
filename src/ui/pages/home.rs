#![allow(non_snake_case)]
use crate::models::app_state::ApplicationData;
use crate::ui::components::item_card_ui::ItemCardUi;
use dioxus::prelude::*;

#[component]
pub fn Home(cx: Scope) -> Element {
    let data = ApplicationData::use_app_data(cx);

    render! {
        div { class: "flex flex-row h-screen w-screen justify-center items-center",
            {
                    data.list.read().iter().enumerate().map(|(id, card)| {
                render!{
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
