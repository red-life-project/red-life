use crate::backend::area::Area;
use crate::backend::screen::{Popup, StackCommand};
use crate::backend::utils::get_scale;
use crate::backend::{error::RLError, screen::Screen};
use crate::game_core::deathscreen::DeathScreen;
use crate::game_core::event::Event;
use crate::game_core::player::Player;
use crate::game_core::resources::Resources;
use crate::{draw, RLResult};
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, Image};
use ggez::graphics::{DrawMode, Mesh, MeshBuilder, Rect};
use ggez::{graphics, Context};
use serde::{Deserialize, Serialize};
use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs;
use std::fs::read_dir;
use std::sync::mpsc::Sender;

const RESOURCE_POSITION: [f32; 3] = [316.0, 639.0, 1373.0];
const RESOURCE_NAME: [&str; 3] = ["Luft", "Energie", "Leben"];
const COLORS: [(u8, u8, u8); 3] = [(51, 51, 204), (186, 158, 19), (102, 24, 18)];
/// This is the game state. It contains all the data that is needed to run the game.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GameState {
    /// Contains the current player position, resources(air, energy, life) and the inventory and their change rates
    pub player: Player,
    /// The current milestone the player has reached.
    milestone: usize,
    machines: Vec<Rect>,
    events: Vec<Event>,
    #[serde(skip)]
    assets: HashMap<String, graphics::Image>,
    #[serde(skip)]
    areas: Vec<Box<dyn Area>>,
    #[serde(skip)]
    screen_sender: Option<Sender<StackCommand>>,
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
    pub fn tick(&mut self) -> Option<StackCommand> {
        // Iterate over every resource and add the change rate to the current value
        self.player.resources = Resources::from_iter(
            self.player
                .resources
                .into_iter()
                .zip(self.player.resources_change.into_iter())
                .map(|(a, b)| a.saturating_add_signed(b)),
        );
        // Check if the player is dead
        if let Some(empty_resource) = Resources::get_zero_values(&self.player.resources) {
            self.player.resources_change.life = -10;
            if self.player.resources.life == 0 {
                let gamestate = GameState::load(true).unwrap_or_default();
                gamestate.save(false).unwrap();
                return Some(StackCommand::Push(Box::new(DeathScreen::new(
                    empty_resource,
                ))));
            };
        }
        None
    }

    /// Paints the current resource level of air, energy and life as a bar on the screen.
    fn draw_resources(&self, canvas: &mut Canvas, scale: Vec2, ctx: &mut Context) -> RLResult {
        self.player
            .resources
            .into_iter()
            .enumerate()
            .map(|(i, resource)| -> RLResult<()> {
                let scale = get_scale(ctx);
                let rect = graphics::Rect::new(
                    RESOURCE_POSITION[i],
                    961.0,
                    resource as f32 * 0.00435,
                    12.6,
                );
                let mesh = graphics::Mesh::new_rounded_rectangle(
                    ctx,
                    DrawMode::fill(),
                    rect,
                    3.0,
                    Color::from(COLORS[i]),
                )?;
                draw!(canvas, &mesh, scale);
                let text = graphics::Text::new(format!(
                    "{}: {:.1}",
                    RESOURCE_NAME[i],
                    (resource as f32 / u16::MAX as f32) * 100.0
                ));
                draw!(
                    canvas,
                    &text,
                    Vec2::new(RESOURCE_POSITION[i] + 20.0, 961.0),
                    scale
                );
                Ok(())
            })
            .for_each(drop);
        Ok(())
    }
    /// Loads the assets. Has to be called before drawing the game.
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

    /// Saves the active game state to a file. The boolean value "milestone" determines whether this is a milestone or an autosave.
    /// If the file already exists, it will be overwritten.
    pub(crate) fn save(&self, milestone: bool) -> RLResult {
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
    /// Loads a game state from a file. The boolean value "milestone" determines whether this is a milestone or an autosave.
    /// If the file doesn't exist, it will return a default game state.
    pub fn load(milestone: bool) -> RLResult<GameState> {
        let save_data = if milestone {
            std::fs::read_to_string("./saves/milestone.yaml")
        } else {
            std::fs::read_to_string("./saves/autosave.yaml")
        }?;
        let game_state: GameState = serde_yaml::from_str(&save_data)?;
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
        next_player_pos.0 >= 1785
            || next_player_pos.1 >= 896
            || next_player_pos.0 <= 280
            || next_player_pos.1 <= 225
    }
    /// Returns a boolean indicating whether the player would collide with a machine or border if they moved in the given direction
    ///
    /// # Arguments
    /// * `next_player_pos` - A tuple containing the next position of the player
    pub(crate) fn collision_detection(&self, next_player_pos: (usize, usize)) -> bool {
        self.machine_collision_detection(next_player_pos)
            || Self::border_collision_detection(next_player_pos)
    }
    /// Returns the asset if it exists
    fn get_asset(&self, name: &str) -> RLResult<&Image> {
        self.assets.get(name).ok_or(RLError::AssetError(format!(
            "Could not find asset with name {}",
            name
        )))
    }
}

impl Screen for GameState {
    /// Updates the game and handles input. Returns StackCommand::Pop when Escape is pressed.
    fn update(&mut self, ctx: &mut Context) -> RLResult<StackCommand> {
        if let Some(death) = self.tick() {
            return Ok(death);
        }
        return self.move_player(ctx);
    }
    /// Draws the game state to the screen.
    fn draw(&self, ctx: &mut Context) -> RLResult {
        let scale = get_scale(ctx);
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));
        let background = self.get_asset("basis.png")?;
        canvas.draw(background, graphics::DrawParam::default().scale(scale));
        let player = self.get_asset("player.png")?;
        draw!(
            canvas,
            player,
            Vec2::from([self.player.position.0 as f32, self.player.position.1 as f32]),
            scale
        );
        self.draw_resources(&mut canvas, scale, ctx)?;
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
