use crate::{error::RedError, screen::Screen};
use ggez::Context;
use serde::{Deserialize, Serialize};

type RedResult = Result<(), RedError>;

#[derive(Clone, Eq, Debug, PartialEq, Serialize, Deserialize)]
struct Item;

#[derive(Clone, Default, Debug, Eq, PartialEq, Serialize, Deserialize)]
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

impl Screen for GameState {
    fn update(&mut self, ctx: &mut Context) -> RedResult {
        self.tick();
        Ok(())
    }
    fn draw(&self, ctx: &mut Context) -> RedResult {
        Ok(())
    }
}
