use crate::screen::StackCommand;
use crate::utils::get_scale;
use crate::{screen::Screen, RLResult};
use ggez::glam::Vec2;
use ggez::graphics::Canvas;
use ggez::winit::event::VirtualKeyCode;
use ggez::{graphics, Context};
use serde::{Deserialize, Serialize};

const MOVEMENT_SPEED: usize = 5;

#[derive(Clone, Eq, Debug, PartialEq, Serialize, Deserialize)]
struct Item;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct Player {
    /// The current items of the player.
    inventory: Vec<Item>,
    position: (usize, usize),
    air: u16,
    energy: u16,
    air_cr: i16,
    energy_cr: i16,
}
impl Default for Player {
    fn default() -> Self {
        Self {
            inventory: vec![],
            position: (0, 0),
            air: u16::MAX,
            energy: u16::MAX,
            air_cr: -10,
            energy_cr: -10
        }
    }
}
/// This is the game state. It contains all the data that is needed to run the game.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct GameState {
    /// Contains the current player position, air and energy
    player: Player,
    /// The current milestone the player has reached.
    milestone: usize,
}


impl GameState {
    pub fn tick(&mut self) {
        self.player.air = self.player.air.saturating_add_signed(self.player.energy_cr);
        self.player.energy = self.player.energy.saturating_add_signed(self.player.air_cr);
        if self.player.air == 0 || self.player.energy == 0 {
            // TODO: Load last game state
            // Remove a milestone if the player is dead
            self.milestone = dbg!(self.milestone.saturating_sub(1));
            self.player.air = u16::MAX;
            self.player.energy = u16::MAX;
        } else {
            self.milestone += 1;
        }
    }

    fn draw_resources(&self, canvas: &mut Canvas, scale: Vec2) -> RLResult {
        let mut text = graphics::Text::new(format!("Air: {}", self.player.air));
        canvas.draw(
            &text,
            graphics::DrawParam::default()
                .dest([50. * scale.x, 1000. * scale.y])
                .scale(scale),
        );
        text = graphics::Text::new(format!("Energy: {}", self.player.energy));
        canvas.draw(
            &text,
            graphics::DrawParam::default()
                .dest([50. * scale.x, 1050. * scale.y])
                .scale(scale),
        );
        Ok(())
    }

    fn save_game_state(&self) -> RLResult {
        let save_data = serde_yaml::to_string(self)?;
        // Create the folder if it doesn't exist
        std::fs::create_dir_all("../saves")?;
        std::fs::write("../saves/savegame.yaml", save_data)?;
        Ok(())
    }

    pub fn load_game_state() -> RLResult<GameState> {
        let save_data = std::fs::read_to_string("../saves/savegame.yaml")?;
        let game_state = serde_yaml::from_str(&save_data)?;
        Ok(game_state)
    }
}

impl Screen for GameState {
    fn update(&mut self, ctx: &mut Context) -> RLResult<StackCommand> {
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
                    dbg!(key);
                }
            }
        }
        Ok(StackCommand::None)
    }

    fn draw(&self, ctx: &mut Context) -> RLResult {
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
