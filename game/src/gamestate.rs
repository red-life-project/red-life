use crate::screen::StackCommand;
use crate::utils::get_scale;
use crate::{screen::Screen, RedResult};
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Transform};
use ggez::winit::event::VirtualKeyCode;
use ggez::{graphics, Context};
use serde::{Deserialize, Serialize};

const MOVEMENT_SPEED: usize = 5;

#[derive(Clone, Eq, Debug, PartialEq, Serialize, Deserialize)]
struct Item;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct Player {
    position: (usize, usize),
    air: u16,
    energy: u16,
}
impl Default for Player {
    fn default() -> Self {
        Self {
            position: (0, 0),
            air: u16::MAX,
            energy: u16::MAX,
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GameState {
    inventory: Vec<Item>,
    player: Player,
    milestone: usize,
    multiplier: u16,
}
impl Default for GameState {
    fn default() -> Self {
        Self {
            inventory: vec![],
            player: Player::default(),
            milestone: 0,
            multiplier: 20,
        }
    }
}

impl GameState {
    pub fn tick(&mut self) {
        self.player.air = self.player.air.saturating_sub(self.multiplier);
        self.player.energy = self.player.energy.saturating_sub(self.multiplier);
        if self.player.air == 0 || self.player.energy == 0 {
            // Remove a milestone if the player is dead
            self.milestone = self.milestone.saturating_sub(1);
            self.player.air = u16::MAX;
            self.player.energy = u16::MAX;
            dbg!("Player died, resetting to milestone: {}", self.milestone);
        } else {
            self.milestone += 1;
        }
    }

    fn draw_resources(&self, canvas: &mut Canvas, scale: Vec2) -> RedResult {
        let mut text = graphics::Text::new(format!("Air: {}", self.player.air));
        canvas.draw(
            &text,
            graphics::DrawParam::default()
                .dest([100. * scale.x, 1000. * scale.y])
                .scale(scale),
        );
        text = graphics::Text::new(format!("Energy: {}", self.player.energy));
        canvas.draw(
            &text,
            graphics::DrawParam::default()
                .dest([100. * scale.x, 1050. * scale.y])
                .scale(scale),
        );
        Ok(())
    }

    fn save_game_state(&self) -> RedResult {
        let save_data = serde_yaml::to_string(self)?;
        // Create the folder if it doesn't exist
        std::fs::create_dir_all("../saves")?;
        std::fs::write("../saves/savegame.yaml", save_data)?;
        Ok(())
    }

    pub fn load_game_state() -> RedResult<GameState> {
        let save_data = std::fs::read_to_string("../saves/savegame.yaml")?;
        let game_state = serde_yaml::from_str(&save_data)?;
        Ok(game_state)
    }
}

impl Screen for GameState {
    fn update(&mut self, ctx: &mut Context) -> RedResult<StackCommand> {
        self.tick();
        let keys = ctx.keyboard.pressed_keys();
        for key in keys.iter() {
            match key {
                VirtualKeyCode::Escape => {
                    self.save_game_state()?;
                    return Ok(StackCommand::Pop);
                }
                VirtualKeyCode::W => {
                    self.player.position.1 = self.player.position.1.saturating_sub(MOVEMENT_SPEED);
                }
                VirtualKeyCode::A => {
                    self.player.position.0 = self.player.position.0.saturating_sub(MOVEMENT_SPEED);
                }
                VirtualKeyCode::S => {
                    self.player.position.1 = self.player.position.1.saturating_add(MOVEMENT_SPEED);
                }
                VirtualKeyCode::D | VirtualKeyCode::Right => {
                    self.player.position.0 = self.player.position.0.saturating_add(MOVEMENT_SPEED);
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
        // TODO: After asset loading we can load this from a hashmap: let background = graphics::Image::from_bytes(ctx, include_bytes!("../../assets/basis.png"))?;
        //canvas.draw(&background, graphics::DrawParam::default().scale(scale));
        let player = graphics::Image::from_bytes(ctx, include_bytes!("../../assets/player.png"))?;
        canvas.draw(
            &player,
            graphics::DrawParam::default()
                .scale(scale)
                .dest([self.player.position.0 as f32, self.player.position.1 as f32]),
        );
        self.draw_resources(&mut canvas, get_scale(ctx))?;
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

    #[test]
    fn test_save_game_state() {
        let gamestate = GameState::default();
        gamestate.save_game_state().unwrap();
    }

    #[test]
    fn test_load_game_state() {
        let gamestate = GameState::load_game_state().unwrap();
    }
}
