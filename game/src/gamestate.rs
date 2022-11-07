use crate::screen::StackCommand;
use crate::utils::get_scale;
use crate::{screen::Screen, RedResult};
use ggez::winit::event::VirtualKeyCode;
use ggez::{graphics, Context};
use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, Debug, PartialEq, Serialize, Deserialize)]
struct Item;

#[derive(Clone, Default, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct Player {
    position: (usize, usize),
}
#[derive(Clone, Default, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GameState {
    inventory: Vec<Item>,
    player: Player,
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
        let keys = ctx.keyboard.pressed_keys();
        for key in keys.iter() {
            match key {
                VirtualKeyCode::Escape => {
                    // TODO: Save the game
                    return Ok(StackCommand::Pop);
                }
                VirtualKeyCode::W => {
                    self.player.position.1 = self.player.position.1.saturating_sub(5);
                }
                VirtualKeyCode::A => {
                    self.player.position.0 = self.player.position.0.saturating_sub(5);
                }
                VirtualKeyCode::S => {
                    self.player.position.1 = self.player.position.1.saturating_add(5);
                }
                VirtualKeyCode::D | VirtualKeyCode::Right => {
                    self.player.position.0 = self.player.position.0.saturating_add(5);
                }
                key => {
                    dbg!("{:?}", key);
                }
            }
        }
        Ok(StackCommand::None)
    }

    fn draw(&self, ctx: &mut Context) -> RedResult {
        let scale = get_scale(ctx);
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));
        // TODO: After asset loading we can do this again: let background = graphics::Image::from_bytes(ctx, include_bytes!("../../assets/basis.png"))?;
        //canvas.draw(&background, graphics::DrawParam::default().scale(scale));
        let player = graphics::Image::from_bytes(ctx, include_bytes!("../../assets/player.png"))?;
        canvas.draw(
            &player,
            graphics::DrawParam::default()
                .scale(scale)
                .dest([self.player.position.0 as f32, self.player.position.1 as f32]),
        );
        canvas.finish(ctx)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gamestate() {
        let gamestate = GameState::default();
    }
}
