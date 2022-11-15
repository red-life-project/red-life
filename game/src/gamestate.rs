use crate::error::RLError;
use crate::screen::StackCommand;
use crate::utils::get_scale;
use crate::{screen::Screen, RLResult};
use ggez::glam::Vec2;
use ggez::graphics::Rect;
use ggez::graphics::{Canvas, Image};
use ggez::winit::event::VirtualKeyCode;
use ggez::{graphics, Context};
use serde::{Deserialize, Serialize};
use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs;
use std::fs::read_dir;

const MOVEMENT_SPEED: usize = 5;
/// Defines an item in the inventory of the player
/// Contains the name of the item, information about the item and the image
#[derive(Clone, Eq, Debug, PartialEq, Serialize, Deserialize)]
struct Item {
    name: String,
    info_text: String,
    //image should be a texture, didnt work yet
    img: String,
}

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
            energy_cr: -10,
        }
    }
}
/// This is the game state. It contains all the data that is needed to run the game.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GameState {
    inventory: Vec<Item>,
    /// Contains the current player position, air and energy
    player: Player,
    /// The current milestone the player has reached.
    milestone: usize,
    machines: Vec<Rect>,
    #[serde(skip)]
    assets: HashMap<String, graphics::Image>,
}
impl PartialEq for GameState {
    fn eq(&self, other: &Self) -> bool {
        self.player == other.player
            && self.milestone == other.milestone
            && self.machines == other.machines
    }
}
impl GameState {
    pub fn new(ctx: &mut Context) -> RLResult<Self> {
        let mut result = GameState::default();
        result.load_assets(ctx)?;
        Ok(result)
    }
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

    /// Draws the current resources to the screen.
    /// TODO: Make them Bars with counters only displayed in debug configurations.
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
    pub(crate) fn load_assets(&mut self, ctx: &mut Context) -> RLResult {
        read_dir("assets")?.for_each(|file| {
            let file = file.unwrap();
            let bytes = fs::read(file.path()).unwrap();
            let name = file.file_name().into_string().unwrap();
            self.assets
                .insert(name, Image::from_bytes(ctx, bytes.as_slice()).unwrap());
        });
        if self.assets.is_empty() {
            return Err(RLError::AssetError("Could not find assets!".to_string()));
        }
        Ok(())
    }

    /// Saves the active game state to a file. The boolean value "milestone" determines whether this is a milestone or an autosave. If the file already exists, it will be overwritten.
    fn save(&self, milestone: bool) -> RLResult {
        let save_data = serde_yaml::to_string(self)?;
        // Create the folder if it doesn't exist
        std::fs::create_dir_all("./saves")?;
        if milestone {
            std::fs::write("./saves/milestone.yaml", save_data)?;
        } else {
            std::fs::write("./saves/autosave.yaml", save_data)?;
        }
        Ok(())
    }
    /// Loads a game state from a file. The boolean value "milestone" determines whether this is a milestone or an autosave. If the file doesn't exist, it will return a default game state.
    pub fn load(milestone: bool) -> RLResult<GameState> {
        let save_data = if milestone {
            std::fs::read_to_string("./saves/milestone.yaml")
        } else {
            std::fs::read_to_string("./saves/autosave.yaml")
        }?;
        let mut game_state: GameState = serde_yaml::from_str(&save_data)?;
        Ok(game_state)
    }

    /// Returns if the player would collide with a machine if they moved in the given direction
    fn machine_collision_detection(&self, next_player_pos: (usize, usize)) -> bool {
        for machine in &self.machines {
            if max(machine.x as usize, next_player_pos.0)
                <= min((machine.x + machine.w) as usize, next_player_pos.0 + 41)
                && max(machine.y as usize, next_player_pos.1)
                    <= min((machine.y + machine.h) as usize, next_player_pos.1 + 50)
            {
                return true;
            }
        }
        false
    }

    /// Returns if the player would collide with a border if they moved in the given direction
    fn border_collision_detection(next_player_pos: (usize, usize)) -> bool {
        next_player_pos.0 >= 1879 || next_player_pos.1 >= 1030
    }
    /// Returns a boolean indicating whether the player would collide with a machine or border if they moved in the given direction
    ///
    /// # Arguments
    /// * `next_player_pos` - A tuple containing the next position of the player
    fn collision_detection(&self, next_player_pos: (usize, usize)) -> bool {
        self.machine_collision_detection(next_player_pos)
            || Self::border_collision_detection(next_player_pos)
    }
}

impl Screen for GameState {
    /// Updates the game and handles input. Returns StackCommand::Pop when Escape is pressed.
    fn update(&mut self, ctx: &mut Context) -> RLResult<StackCommand> {
        self.tick();
        let keys = ctx.keyboard.pressed_keys();
        for key in keys.iter() {
            match key {
                VirtualKeyCode::Escape => {
                    self.save(false)?;
                    return Ok(StackCommand::Pop);
                }
                VirtualKeyCode::W => {
                    if !self.collision_detection((
                        self.player.position.0,
                        self.player.position.1.saturating_sub(MOVEMENT_SPEED),
                    )) {
                        self.player.position.1 =
                            self.player.position.1.saturating_sub(MOVEMENT_SPEED);
                    }
                }
                VirtualKeyCode::A => {
                    if !self.collision_detection((
                        self.player.position.0.saturating_sub(MOVEMENT_SPEED),
                        self.player.position.1,
                    )) {
                        self.player.position.0 =
                            self.player.position.0.saturating_sub(MOVEMENT_SPEED);
                    }
                }
                VirtualKeyCode::S => {
                    if !self.collision_detection((
                        self.player.position.0,
                        self.player.position.1.saturating_add(MOVEMENT_SPEED),
                    )) {
                        self.player.position.1 =
                            self.player.position.1.saturating_add(MOVEMENT_SPEED);
                    }
                }
                VirtualKeyCode::D => {
                    if !self.collision_detection((
                        self.player.position.0.saturating_add(MOVEMENT_SPEED),
                        self.player.position.1,
                    )) {
                        self.player.position.0 =
                            self.player.position.0.saturating_add(MOVEMENT_SPEED);
                    }
                }
                key => {
                    dbg!(key);
                }
            }
        }
        Ok(StackCommand::None)
    }
    /// Draws the game state to the screen.
    fn draw(&self, ctx: &mut Context) -> RLResult {
        let scale = get_scale(ctx);
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));
        let background = self
            .assets
            .get("basis.png")
            .ok_or(RLError::AssetError("Could not find asset".to_string()))?;
        canvas.draw(background, graphics::DrawParam::default().scale(scale));
        let player = self
            .assets
            .get("player.png")
            .ok_or(RLError::AssetError("Could not find asset".to_string()))?;
        canvas.draw(
            player,
            graphics::DrawParam::default()
                .scale(scale)
                .dest([self.player.position.0 as f32, self.player.position.1 as f32]),
        );
        self.draw_resources(&mut canvas, scale)?;
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
    fn test_save_autosave() {
        let gamestate = GameState::default();
        gamestate.save(false).unwrap();
    }

    #[test]
    fn test_save_milestone() {
        let gamestate = GameState::default();
        gamestate.save(true).unwrap();
    }

    #[test]
    fn test_load_autosave() {
        let gamestate = GameState::load(false).unwrap();
    }

    #[test]
    fn test_load_milestone() {
        let gamestate = GameState::load(true).unwrap();
    }
}
