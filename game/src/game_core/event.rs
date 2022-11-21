use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Event {
    name: String,
    rarity: u8,
    info_text: String,
    duration: String,
}
impl Event {
    pub fn new(name: String, rarity: u8, info_text: String, duration: String) -> Self {
        Self {
            name,
            rarity,
            info_text,
            duration,
        }
    }
}
