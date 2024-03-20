use super::item::ItemCard;
use dioxus::prelude::*;

#[derive(Clone, Copy, Default)]
pub struct ApplicationData {
    pub list: Signal<Vec<ItemCard>>,
    pub currentCard: Signal<usize>,
}

impl ApplicationData {
    pub fn new() -> Self {
        Self {
            list: Signal::new(vec![] as Vec<ItemCard>),
            currentCard: Signal::new(0 as usize),
        }
    }
}
