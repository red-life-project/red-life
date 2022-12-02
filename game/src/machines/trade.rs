use crate::game_core::item::Item;

use crate::machines::machine::State;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Trade {
    pub(crate) name: String,
    pub time_ticks: i16,
    pub initial_state: State,
    pub resulting_state: State,
    // the Machine needs to be in this state for the trade to be accessible
    //ggf eine weitere State in was dieser trade die maschiene ändert
    pub(crate) cost: Vec<(Item, i32)>,
    result: Item,
    amount_produced: usize,
}

impl Default for Trade {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            time_ticks: 0,
            initial_state: State::Broken,
            resulting_state: State::Running,
            cost: vec![],
            result: Item::default(),
            amount_produced: 0,
        }
    }
}

impl Trade {
    pub fn new(
        name: String,
        time_ticks: i16,
        initial_state: State,
        resulting_state: State,
        cost: Vec<(Item, i32)>,
        result: Item,
        amount_produced: usize,
    ) -> Self {
        Self {
            name,
            time_ticks,
            initial_state,
            resulting_state,
            cost,
            result,
            amount_produced,
        }
    }
}
