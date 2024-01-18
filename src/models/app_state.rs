use super::item::ItemCard;
use dioxus::prelude::*;
use dioxus_signals::*;

#[derive(Clone, Copy, Default)]
pub struct ApplicationData {
    pub list: Signal<Vec<ItemCard>>,
    pub currentCard: Signal<usize>,
}

impl ApplicationData {
    pub fn new(list: Signal<Vec<ItemCard>>, currentCard: Signal<usize>) -> Self {
        Self { list, currentCard }
    }

    pub fn use_app_data(cx: &ScopeState) -> ApplicationData {
        *use_context(cx).unwrap()
    }
}
