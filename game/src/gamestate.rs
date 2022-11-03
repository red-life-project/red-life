use crate::{RedResult, screen::Screen};
use ggez::{Context, graphics};
use serde::{Deserialize, Serialize};
use crate::screen::StackCommand;
use crate::utils::get_scale;


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
    fn update(&mut self, ctx: &mut Context) -> RedResult<StackCommand> {
        self.tick();
        Ok(StackCommand::None)
    }
    fn draw(&self, ctx: &mut Context) -> RedResult {
        let scale = get_scale(ctx);
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));
        let background =
            graphics::Image::from_bytes(ctx, include_bytes!("../../assets/basis.png"))?;
        canvas.draw(&background, graphics::DrawParam::default().scale(scale));
        canvas.finish(ctx)?;
        Ok(())
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gamestate() {
        let mut gamestate = GameState::default();

    }
}