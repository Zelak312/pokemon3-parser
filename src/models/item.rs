use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Item {
    pub id: u16,
    pub name: String,
    pub quantity: u16,
}

impl Item {
    pub fn new(id: u16, quant: u16) -> Self {
        Self {
            id,
            name: String::from("None"),
            quantity: quant,
        }
    }
}
