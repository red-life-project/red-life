//! This File contains the structure `Trade`
use crate::game_core::item::Item;
use crate::machines::machine::State;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// A trade is one interaction option with a machine
/// This means that repairing, starting or pausing a machine are all trades
/// Trade itself does not contain any logic and is mostly a construct to group values
pub struct Trade {
    /// is used for debugging and login purposes
    pub(crate) name: String,
    /// the time it takes for the trade to conclude 0 = instant
    pub time_ticks: i16,
    /// the Machine needs to be in `initial_state` for the trade to be accessible
    pub initial_state: State,
    /// determines the machine state after or during the trade
    pub resulting_state: State,
    /// determines whether the `resulting_state` is temporary or permanent
    /// * true = temporary, meaning the state returns to `initial_state` after the timer
    /// + false = permanent, meaning the state is set to `resulting_state` after the timer
    pub return_after_timer: bool, // how the ms behaves after the timer run out
    /// Contains the cost associated with this trade.
    /// This stores the amount of item the player loses and or gain.
    /// * Positive amount means the Player will **lose** these items.
    /// * Negative amount means the Player will **gain** these items.
    pub(crate) cost: Vec<(Item, i32)>,
}

impl Default for Trade {
    /// Initialises a trade with some  default values
    /// default values have no meaning and should never be checked on
    fn default() -> Self {
        Self {
            name: "no_Trade".to_string(),
            time_ticks: 0,
            initial_state: State::Broken,
            resulting_state: State::Running,
            return_after_timer: false,
            cost: vec![],
        }
    }
}

impl Trade {
    ///initialises a new Trade using values passed in
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
