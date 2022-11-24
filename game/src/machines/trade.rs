use crate::game_core::item::Item;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Trade {
    time_ms: usize,
    cost: Vec<(Item, usize)>,
    result: Item,
    amount_produced: usize,
}

impl Default for Trade {
    fn default() -> Self {
        Self {
            time_ms: 1000,
            cost: Vec::default(),
            result: Item::default(),
            amount_produced: 0,
        }
    }
}
