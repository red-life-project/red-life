use crate::game_core::item::Item;

use crate::machines::machine::State;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Trade {
    pub(crate) name: String,
    time_ms: usize,
    pub ms_state: State, // the Machine needs to be in this state for the trade to be accessible
    //ggf eine weitere State in was dieser trade die maschiene ändert
    pub(crate) cost: Vec<(Item, i32)>,
    result: Item,
    amount_produced: usize,
}

impl Default for Trade {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            time_ms: 1000,
            ms_state: State::Broken,
            cost: vec![],
            result: Item::default(),
            amount_produced: 0,
        }
    }
}

impl Trade {
    pub fn new(
        name: String,
        time_ms: usize,
        ms_state: State,
        cost: Vec<(Item, i32)>,
        result: Item,
        amount_produced: usize,
    ) -> Self {
        Self {
            name,
            time_ms,
            ms_state,
            cost,
            result,
            amount_produced,
        }
    }
    pub fn new_and_set(
        name: String,
        time_ms: usize,
        ms_state: State,
        cost: Vec<(Item, i32)>,
        (a, b, c): (i32, i32, i32),
        result: Item,
        amount_produced: usize,
    ) -> Self {
        cost[0].1 = a;
        cost[1].1 = b;
        cost[2].1 = c;
        Self {
            name,
            time_ms,
            ms_state,
            cost,
            result,
            amount_produced,
        }
    }
}
