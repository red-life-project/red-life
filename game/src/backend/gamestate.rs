//! Contains the game logic, updates the game and draws the current board
use crate::backend::area::Area;
use crate::backend::rlcolor::RLColor;
use crate::backend::screen::StackCommand;
use crate::backend::utils::{get_scale, is_colliding};
use crate::backend::{error::RLError, screen::Screen};
use crate::game_core::deathscreen::DeathReason::Both;
use crate::game_core::deathscreen::DeathScreen;
use crate::game_core::event::Event;
use crate::game_core::player::Player;
use crate::game_core::resources::Resources;
use crate::{draw, RLResult};
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, Image};
use ggez::graphics::{DrawMode, Mesh, Rect};
use ggez::{graphics, Context};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::read_dir;
use std::ops::Deref;
use std::sync::mpsc::Sender;
use tracing::info;
const RESOURCE_POSITION: [f32; 3] = [316.0, 639.0, 1373.0];
const RESOURCE_NAME: [&str; 3] = ["Luft", "Energie", "Leben"];
const COLORS: [Color; 3] = [RLColor::BLUE, RLColor::GOLD, RLColor::DARK_RED];
/// This is the game state. It contains all the data that is needed to run the game.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GameState {
    /// Contains the current player position, resources(air, energy, life) and the inventory and their change rates
    pub player: Player,
    events: Option<Event>,
    #[serde(skip)]
    assets: HashMap<String, Image>,
    #[serde(skip)]
    pub areas: Vec<Box<dyn Area>>,
    #[serde(skip)]
    pub(crate) screen_sender: Option<Sender<StackCommand>>,
}

impl PartialEq for GameState {
    fn eq(&self, other: &Self) -> bool {
        self.player == other.player && self.player.milestone == other.player.milestone
    }
}

impl GameState {
    /// Creates a new game state at the beginning of the game and after every loading.
    /// It loads all the assets and creates the areas of the machines.
    pub fn new(ctx: &mut Context) -> RLResult<Self> {
        info!("Creating new gamestate");
        let mut result = GameState::default();
        result.load_assets(ctx)?;
        result.create_machine()?; //////////// SANDER TESTING TOBE RM
        Ok(result)
    }
    /// Gets called every tick in the update fn to update the internal game logic.
    /// It updates the player resources, checks on the current milestone if the player has reached a new one
    /// and checks if the player has died.
    pub fn tick(&mut self, ctx: &mut Context) -> RLResult {
        // Iterate over every resource and add the change rate to the current value
        self.get_current_milestone(ctx);
        self.player.resources = self
            .player
            .resources
            .into_iter()
            .zip(self.player.resources_change.into_iter())
            .map(|(a, b)| a.saturating_add_signed(b))
            .collect::<Resources<_>>();
        // Check if player is able to regenerate life
        self.player
            .life_regeneration(&self.screen_sender.as_ref().unwrap().clone());
        // Check if the player is dead
        if let Some(empty_resource) = Resources::get_death_reason(&self.player.resources) {
            match empty_resource {
                Both => self.player.resources_change.life = -20,
                _ => self.player.resources_change.life = -10,
            }
            if self.player.resources.life == 0 {
                let gamestate = GameState::load(true).unwrap_or_default();
                gamestate.save(false).unwrap();
                let cloned_sender = self.screen_sender.as_mut().unwrap().clone();
                self.screen_sender
                    .as_mut()
                    .expect("No screen sender")
                    .send(StackCommand::Push(Box::new(DeathScreen::new(
                        empty_resource,
                        cloned_sender,
                    ))))?;
            };
        }
        Ok(())
    }

    /// Paints the current resource level of air, energy and life as a bar on the screen and
    /// draws the amount of every resource in the inventory.
    fn draw_resources(&self, canvas: &mut Canvas, scale: Vec2, ctx: &mut Context) {
        self.player
            .resources
            .into_iter()
            .enumerate()
            .map(|(i, resource)| -> RLResult<()> {
                let mut color = COLORS[i];
                if i == 2 && self.player.resources_change.life > 0 {
                    color = RLColor::GREEN;
                };
                let rect = Rect::new(RESOURCE_POSITION[i], 961.0, resource as f32 * 0.00435, 12.6);
                let mesh = Mesh::new_rounded_rectangle(ctx, DrawMode::fill(), rect, 3.0, color)?;
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
    }
    /// iterates trough the inventory and draws the amount of every item in the inventory.
    fn draw_items(&self, canvas: &mut Canvas, ctx: &mut Context) {
        self.player
            .inventory
            .clone()
            .into_iter()
            .enumerate()
            .map(|(i, (item, amount))| {
                let img = self.assets.get(item.img.as_str()).unwrap();
                let position = (990., 955.);
                let scale = get_scale(ctx);
                draw!(
                    canvas,
                    img,
                    Vec2::new(position.0 + (i * 65) as f32, position.1),
                    scale
                );
                draw!(
                    canvas,
                    &graphics::Text::new(format!("{amount}")),
                    Vec2::new(position.0 + (i * 63) as f32, position.1),
                    scale
                );
            })
            .for_each(drop);
    }
    /// Loads the assets. Has to be called before drawing the game.
    pub(crate) fn load_assets(&mut self, ctx: &mut Context) -> RLResult {
        info!("Loading assets");
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
    /// # Arguments
    /// * `milestone` - Boolean value that determines whether this is a milestone save or an autosave.
    pub(crate) fn save(&self, milestone: bool) -> RLResult {
        let save_data = serde_yaml::to_string(self)?;
        // Create the folder if it doesn't exist
        fs::create_dir_all("./saves")?;
        if milestone {
            fs::write("./saves/milestone.yaml", save_data)?;
            info!("Saved gamestate as milestone");
        } else {
            fs::write("./saves/autosave.yaml", save_data)?;
            info!("Saved gamestate as autosave");
        }
        Ok(())
    }
    /// Loads a game state from a file. The boolean value "milestone" determines whether this is a milestone or an autosave.
    /// If the file doesn't exist, it will return a default game state.
    /// # Arguments
    /// * `milestone` - Whether to load the milestone or the autosave
    pub fn load(milestone: bool) -> RLResult<GameState> {
        let save_data = if milestone {
            info!("Loading milestone...");
            fs::read_to_string("./saves/milestone.yaml")
        } else {
            info!("Loading autosave...");
            fs::read_to_string("./saves/autosave.yaml")
        }?;
        let game_state: GameState = serde_yaml::from_str(&save_data)?;
        Ok(game_state)
    }
    /// Returns the area the player needs to stand in to interact with a machine
    pub(crate) fn get_interactable(&mut self) -> Option<&mut Box<dyn Area>> {
        self.areas
            .iter_mut()
            .find(|area| area.is_interactable(self.player.position))
    }

    /// Returns if the player would collide with a border if they moved in the given direction
    /// # Arguments
    /// * `next_player_pos` - The direction the player wants to move
    fn border_collision_detection(next_player_pos: (usize, usize)) -> bool {
        next_player_pos.0 >= 1750 // Right border
            || next_player_pos.1 >= 850 // Bottom border
            || next_player_pos.0 <= 255 // Left border
            || next_player_pos.1 <= 220 // Top border
    }
    /// Returns a boolean indicating whether the player would collide with a machine or border if they moved in the given direction
    ///
    /// # Arguments
    /// * `next_player_pos` - A tuple containing the next position of the player
    pub(crate) fn collision_detection(&self, next_player_pos: (usize, usize)) -> bool {
        self.areas
            .iter()
            .map(|area| area.get_collision_area())
            .any(|area| is_colliding(next_player_pos, &area))
            || Self::border_collision_detection(next_player_pos)
    }
    /// Returns the asset if it exists
    /// # Arguments
    /// * `name` - The name of the asset
    pub fn get_asset(&self, name: &str) -> RLResult<&Image> {
        self.assets
            .get(name)
            .ok_or_else(|| RLError::AssetError(format!("Could not find asset with name {}", name)))
    }
    /// Checks if the milestone is reached which means the vec of repaired machines
    /// contain the vec of machines needed to reach the next milestone.
    /// # Arguments
    /// * `milestone_machines` - A vec of machines needed to reach the next milestone
    pub fn check_on_milestone(&mut self, milestone_machines: &[String]) {
        //let a = self.areas.get(0).unwrap().deref(); erst einfügen, wenn man es auch benutzt

        let running_machine = self
            .areas
            .iter()
            .map(Deref::deref)
            .filter(|m| m.is_non_broken_machine())
            .map(Area::get_name)
            .collect::<Vec<String>>();

        if !running_machine.is_empty() {
            info!("found running_machines len: {}", running_machine.len());
        }

        if milestone_machines
            .iter()
            .all(|machine| running_machine.contains(&machine.to_string()))
        {
            self.player.milestone += 1;
            info!("Player reached milestone {}", self.player.milestone);
            self.save(true).unwrap();
        }
    }
    /// Decides what happens if a certain milestone is reached
    /// divided into 3 milestones
    fn get_current_milestone(&mut self, ctx: &mut Context) {
        match self.player.milestone {
            1 => {
                if self.player.match_milestone == 0 {
                    self.player.resources_change.oxygen = -1;
                    self.player.resources_change.energy = -1;
                    self.player.last_damage = 0;
                    self.events = None;
                    self.player.match_milestone = 1;
                }
                if ctx.time.ticks() % 5000 == 0 {
                    if self.events.is_none() {
                        self.events =
                            Event::event_generator(&self.screen_sender.as_ref().unwrap().clone());
                    } else {
                        self.events = Event::restore_event();
                    }
                }
                self.check_on_milestone(&[
                    "Sauerstoffgenerator".to_string(),
                    "Stromgenerator".to_string(),
                ]);
            }
            2 => {
                self.check_on_milestone(&["Kommunikationsmodul".to_string()]);
            }
            _ => {}
        }
    }
    /// Deletes all files in the directory saves, returns Ok if saves directory does not exist
    pub(crate) fn delete_saves() -> RLResult {
        info!("deleting saves");
        let existing_files = fs::read_dir("./saves");
        if existing_files.is_err() {
            return Ok(());
        }
        for entry in existing_files? {
            let file = entry?;
            if file.metadata()?.is_file() {
                fs::remove_file(file.path())?;
            }
        }
        Ok(())
    }
}

impl Screen for GameState {
    /// Updates the game and handles input. Returns `StackCommand::Pop` when Escape is pressed.
    fn update(&mut self, ctx: &mut Context) -> RLResult {
        const DESIRED_FPS: u32 = 60;
        if ctx.time.check_update_time(DESIRED_FPS) {
            self.tick(ctx)?;
            self.move_player(ctx)?;
        }
        Ok(())
    }
    /// Draws the game state to the screen.
    fn draw(&self, ctx: &mut Context) -> RLResult {
        let scale = get_scale(ctx);
        let mut canvas = Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));
        let background = self.get_asset("basis.png")?;
        canvas.draw(background, graphics::DrawParam::default().scale(scale));
        let player = self.get_asset("player.png")?;
        draw!(
            canvas,
            player,
            Vec2::from([self.player.position.0 as f32, self.player.position.1 as f32]),
            scale
        );
        self.draw_resources(&mut canvas, scale, ctx);
        self.draw_machines(&mut canvas, scale);
        self.draw_items(&mut canvas, ctx);
        #[cfg(debug_assertions)]
        {
            let fps = graphics::Text::new(format!("FPS: {}", ctx.time.fps()));
            draw!(canvas, &fps, Vec2::new(0.0, 0.0), scale);
        }

        canvas.finish(ctx)?;
        Ok(())
    }
    fn set_sender(&mut self, sender: Sender<StackCommand>) {
        self.screen_sender = Some(sender);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gamestate() {
        let _gamestate = GameState::default();
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
        GameState::default().save(false).unwrap();
        let _gamestate_loaded = GameState::load(false).unwrap();
    }

    #[test]
    fn test_load_milestone() {
        GameState::default().save(true).unwrap();
        let _gamestate_loaded = GameState::load(true).unwrap();
    }
    #[test]
    fn test_delete_saves() {
        GameState::delete_saves().unwrap();
    }
}
