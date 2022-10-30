mod error;
mod gamestate;
mod screen;
mod mainmenu;

use crate::gamestate::GameState;
use ggez::{
    event,
    graphics::{self, Color},
};


pub fn main() {
    let cb = ggez::ContextBuilder::new("red-life", "red-life-project");
    let (mut ctx, event_loop) = cb.build().unwrap();
    let state = GuiState::new(&mut ctx).unwrap();
    event::run(ctx, event_loop, state);
}
