use std::collections::HashMap;
use std::path::Iter;
use ggez::glam::Vec2;
use super::super::game_core::item;

struct Trade {
    time_ms: usize,
    cost: HashMap<item, usize>,
    result: item,
    amount_produced: usize,
}

impl Default for Trade {
    fn default() -> Self {

        Self{
            time_ms:1000,
            cost:HashMap::default(),
            result:item,
            amount_produced: 0
        }
    }
}