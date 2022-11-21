use crate::game_core::item::Item;
use ggez::glam::Vec2;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Iter;

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
