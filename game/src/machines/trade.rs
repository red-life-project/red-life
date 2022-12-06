use crate::game_core::item::Item;
use crate::machines::machine::State;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Trade {
    pub(crate) name: String,
    pub time_ticks: i16,
    pub initial_state: State, // the Machine needs to be in this state for the trade to be accessible
    pub resulting_state: State, // the Machine changes state to resulting state after pressing E
    pub return_after_timer: bool, // how the ms behaves after the timer run out
    pub(crate) cost: Vec<(Item, i32)>,
}

impl Default for Trade {
    fn default() -> Self {
        Self {
            name: "no_Trade".to_string(),
            // default values have almost no meaning
            time_ticks: 0,
            initial_state: State::Broken,
            resulting_state: State::Running,
            return_after_timer: false,
            cost: vec![],
        }
    }
}

impl Trade {
    pub fn new(
        name: String,
        time_ticks: i16,
        initial_state: State,
        resulting_state: State,
        return_after_timer: bool,
        cost: Vec<(Item, i32)>,
    ) -> Self {
        Self {
            name,
            time_ticks,
            initial_state,
            resulting_state,
            return_after_timer,
            cost,
        }
    }
}
