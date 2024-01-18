#[derive(PartialEq, Debug, Clone, Copy)]
pub struct ItemCard {
    pub id: usize,
    pub draggable: bool,
    pub text: &'static str,
    pub color: &'static str,
}

impl ItemCard {
    pub fn new(id: usize, text: &'static str, color: &'static str) -> Self {
        Self {
            id,
            draggable: true,
            text,
            color,
        }
    }
}
