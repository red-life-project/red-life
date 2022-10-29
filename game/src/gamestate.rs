use std::time::Duration;
use serde::{Serialize, Deserialize};

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
struct Item;
#[derive(Clone, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct GameState {
    inventory: Vec<Item>,
    milestone: usize,
}
impl GameState {
    pub fn tick(&mut self) {
        // do stuff
        self.milestone += 1;
    }
}
