//! Contains the game logic, updates the game and draws the current board
use crate::backend::constants::{
    ObjectId, COLORS, DESIRED_FPS, MAP_BORDER, RESOURCE_POSITION, TIME_POSITION,
};
use crate::backend::rlcolor::RLColor;
use crate::backend::screen::{Popup, ScreenCommand, StackCommand};
use crate::backend::utils::get_scale;
use crate::backend::utils::{get_draw_params, is_colliding};
use crate::backend::{error::RLError, screen::Screen};
use crate::game_core::event::Event;
use crate::game_core::infoscreen::DeathReason::{Both, Energy, Oxygen};
use crate::game_core::infoscreen::InfoScreen;
use crate::game_core::item::Item;
use crate::game_core::player::Player;
use crate::game_core::resources::Resources;
use crate::languages::{
    first_milestone_handbook_text, resource_name, second_milestone_handbook_text, send_msg_failure,
    time_name, Lang,
};
use crate::machines::machine::Machine;
use crate::machines::machine::State::Broken;
use crate::{draw, RLResult};
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Image, TextFragment};
use ggez::graphics::{DrawMode, Mesh, Rect};
use ggez::{graphics, Context};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::read_dir;
use std::path::Path;
use std::sync::mpsc::{channel, Receiver, Sender};
use tracing::{debug, info};

pub enum GameCommand {
    AddItems(Vec<(Item, i32)>),
    ResourceChange(Resources<i16>),
    Milestone,
    Winning,
}

/// This is the game state. It contains all the data that is needed to run the game.
#[derive(Debug, Serialize, Deserialize)]
pub struct GameState {
    /// Contains the current player position, resources(air, energy, life) and the inventory and their change rates
    pub player: Player,
    /// Contains the event generator and the current events
    pub(crate) events: Vec<Event>,
    /// Contains the machines and their current state
    pub machines: Vec<Machine>,
    #[serde(skip)]
    /// Contains all the images that are needed to draw the game on the canvas
    assets: HashMap<String, Image>,
    #[serde(skip)]
    /// Needed to send Messages to the `Screenstack` to make changes to the screen
    pub(crate) screen_sender: Option<Sender<StackCommand>>,
    #[serde(skip)]
    /// Needed to receive Messages from `machine` to make changes to the game
    pub(crate) receiver: Option<Receiver<GameCommand>>,
    #[serde(skip)]
    /// Needed to send Messages to `machine` to make changes to the game
    pub(crate) sender: Option<Sender<GameCommand>>,
    /// Defines if the handbook is currently open
    pub handbook_invisible: bool,
    #[serde(default)]
    pub lng: Lang,
}

impl PartialEq for GameState {
    /// Compares the game state by comparing the player, events and machines
    fn eq(&self, other: &Self) -> bool {
        self.player == other.player && self.player.milestone == other.player.milestone
    }
}

impl GameState {
    pub fn new_with_lang(lng: Lang) -> Self {
        Self {
            player: Player::new(lng),
            events: vec![],
            machines: vec![],
            assets: HashMap::with_capacity(64),
            screen_sender: None,
            receiver: None,
            sender: None,
            handbook_invisible: false,
            lng,
        }
    }

    /// Gets the screen sender
    /// # Returns
    /// * `RLResult<Sender<StackCommand>>`: The screen sender in a `RLResult` to handle Initialization errors
    pub(crate) fn get_screen_sender(&mut self) -> RLResult<&mut Sender<StackCommand>> {
        self.screen_sender.as_mut().ok_or(RLError::InitError(
            "No Screen Sender found. The game was not initialized properly".to_string(),
        ))
    }
    /// Gets the receiver
    /// # Returns
    /// * `RLResult<Receiver<GameCommand>>`: The receiver in a `RLResult` to handle Initialization errors
    pub(crate) fn get_receiver(&mut self) -> RLResult<&Receiver<GameCommand>> {
        self.receiver.as_ref().ok_or(RLError::InitError(
            "No Receiver found. The game was not initialized properly".to_string(),
        ))
    }

    /// Creates a new game state at the beginning of the game and after every loading.
    /// It loads all the assets and creates the areas of the machines.
    /// # Returns
    /// * `RLResult<GameState>`: The new game state initialized in a `RLResult` to handle setup errors
    pub fn new(ctx: &mut Context, lng: Lang) -> RLResult<Self> {
        info!("Creating new gamestate");
        let (sender, receiver) = channel();
        let mut result = GameState {
            player: Player::new(lng),
            events: vec![],
            machines: vec![],
            assets: HashMap::with_capacity(64),
            sender: Some(sender),
            receiver: Some(receiver),
            lng,
            screen_sender: None,
            handbook_invisible: false,
        };
        result.init(ctx)?;
        Ok(result)
    }
    /// Gets called every tick in the update fn to update the internal game logic.
    /// It updates the player resources, checks on the current milestone if the player has reached a new one
    /// and checks if the player has died.
    /// # Returns
    /// * `RLResult`: A `RLResult` to validate the success of the tick function
    pub fn tick(&mut self) -> RLResult {
        let lng = self.lng;

        // Update Resources
        self.player.resources = self
            .player
            .resources
            .into_iter()
            .zip(self.player.resources_change.into_iter())
            .map(|(a, b)| a.saturating_add_signed(b))
            .collect::<Resources<_>>();
        self.player.time += 1;
        // Everything inside will only be checked every 15 ticks

        // Check if the player is dead
        if let Some(empty_resource) = Resources::get_death_reason(self.player.resources) {
            match empty_resource {
                Both => {
                    self.player.resources_change.life = -60;
                    self.machines.iter_mut().for_each(Machine::no_energy);
                }
                Oxygen => self.player.resources_change.life = -50,
                Energy => {
                    self.player.resources_change.life = -10;
                    self.machines.iter_mut().for_each(Machine::no_energy);
                }
            };
            if self.player.resources.life == 0 {
                let game_state =
                    GameState::load(true).unwrap_or_else(|_| GameState::new_with_lang(lng));
                game_state.save(false)?;
                let cloned_sender = self.get_screen_sender()?.clone();
                self.get_screen_sender()?
                    .send(StackCommand::Screen(ScreenCommand::Push(Box::new(
                        InfoScreen::new_death_screen(empty_resource, cloned_sender, game_state.lng),
                    ))))?;
            };
        } else if self.player.resources_change.life < 0 {
            self.player.resources_change.life = 0;
        }

        // process received GameCommands
        if let Ok(msg) = self.get_receiver()?.try_recv() {
            match msg {
                GameCommand::ResourceChange(new_rs) => {
                    self.player.resources_change = self.player.resources_change + new_rs;
                }
                GameCommand::AddItems(items) => {
                    for (item, amount) in &items {
                        self.player.add_item(item, *amount);
                    }
                }
                GameCommand::Milestone => {
                    self.get_current_milestone()?;
                }
                GameCommand::Winning => match self.player.milestone {
                    1 => {
                        let sender = self.get_screen_sender()?;
                        let popup =
                            Popup::new(RLColor::GREEN, send_msg_failure(lng).into(), 5);
                        sender.send(StackCommand::Screen(ScreenCommand::Popup(popup)))?;
                    }
                    2 => {
                        self.player.milestone += 1;
                        self.get_current_milestone()?;
                    }
                    _ => {}
                },
            };
        }

        // Regenerate life if applicable
        self.player
            .life_regeneration(&self.screen_sender.as_ref().unwrap().clone(), lng)?;
        for machine in &mut self.machines {
            machine.tick()?;
        }

        Ok(())
    }

    /// Paints the current resource level of air, energy and life as a bar on the screen and
    /// draws the amount of every resource in the inventory.
    /// # Arguments
    /// * `canvas`: The canvas to draw on
    /// * `scale`: The scale of the canvas
    /// * `ctx`: The `Context` of the game
    /// # Returns
    /// * `RLResult`: A `RLResult` to validate the success of the paint function
    fn draw_resources(&self, canvas: &mut Canvas, scale: Vec2, ctx: &mut Context) {
        self.player
            .resources
            .into_iter()
            .enumerate()
            .map(|(i, resource)| -> RLResult<()> {
                let resource = f64::from(resource);
                let mut color = COLORS[i];
                if i == 2 && self.player.resources_change.life > 0 {
                    color = RLColor::GREEN;
                };
                let rect = Rect::new(
                    RESOURCE_POSITION[i],
                    961.0,
                    (resource * 0.00435_f64) as f32,
                    12.6,
                );
                let mesh = Mesh::new_rounded_rectangle(ctx, DrawMode::fill(), rect, 3.0, color)?;
                draw!(canvas, &mesh, scale);
                let text = graphics::Text::new(format!(
                    "{}: {:.1}",
                    resource_name(self.lng)[i],
                    (resource / f64::from(u16::MAX)) * 100.0
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
    /// Draws the handbook while pressing the H key
    /// # Arguments
    /// * `canvas`: The canvas to draw on
    /// * `ctx`: The `Context` of the game
    /// # Returns
    /// * `RLResult`: A `RLResult` to validate the success of the function
    pub fn open_handbook(&self, canvas: &mut Canvas, ctx: &mut Context) {
        let scale = get_scale(ctx);
        let image = self.assets.get("Handbook.png").unwrap();
        draw!(canvas, image, Vec2::new(700.0, 300.0), scale);
        match self.player.milestone {
            1 => {
                Self::draw_handbook_text(canvas, scale, first_milestone_handbook_text(self.lng));
            }

            2 => {
                Self::draw_handbook_text(canvas, scale, second_milestone_handbook_text(self.lng));
            }
            _ => {}
        }
    }
    /// Draws the text for the current milestone on the handbook on the screen.
    /// # Arguments
    /// * `canvas`: The canvas to draw on
    /// * `scale`: The scale of the canvas
    /// * `handbook_text`: The text to draw on the screen
    /// # Returns
    /// * `RLResult`: A `RLResult` to validate the success of the function
    pub fn draw_handbook_text(canvas: &mut Canvas, scale: Vec2, handbook_text: &[&str]) {
        handbook_text
            .iter()
            .enumerate()
            .for_each(|(i, const_text)| {
                let mut text =
                    graphics::Text::new(TextFragment::new(*const_text).color(RLColor::BLACK));
                text.set_scale(28.0);
                draw!(
                    canvas,
                    &text,
                    Vec2::new(800.0, 400.0 + (i * 30) as f32),
                    scale
                );
            });
    }

    /// Iterates trough the inventory and draws the amount of every item in the inventory.
    /// # Arguments
    /// * `canvas` - The current canvas to draw on
    /// * `ctx` - The current game context
    /// # Returns
    /// * `RLResult` - validates if the drawing was successful
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

    /// Draws the current time on the screen
    /// # Arguments
    /// * `canvas` - The current canvas to draw on
    /// * `scale` - The current scale of the canvas
    pub(crate) fn draw_time(&self, canvas: &mut Canvas, scale: Vec2) {
        let time = self.player.time / DESIRED_FPS;
        let time_text = format!(
            "{}: {}h {}m {}s",
            time_name(self.lng)[0],
            time / 3600,
            time / 60,
            time % 60
        );
        let mut text = graphics::Text::new(TextFragment::new(time_text).color(RLColor::BLACK));
        text.set_scale(18.0);
        draw!(
            canvas,
            &text,
            Vec2::new(TIME_POSITION.0, TIME_POSITION.1),
            scale
        );
    }
    /// Loads the assets. Has to be called before drawing the game.
    /// # Returns
    /// * `RLResult` - Returns an error if the assets could not be loaded.
    pub(crate) fn init(&mut self, ctx: &mut Context) -> RLResult {
        info!("Loading assets");
        read_dir("assets")?.for_each(|file| {
            debug!("Loading asset: {:?}", file);
            let file = file.unwrap();
            if file.file_name().to_str().unwrap().ends_with(".png") {
                let bytes = fs::read(file.path()).unwrap();
                let name = file.file_name().into_string().unwrap();
                self.assets
                    .insert(name, Image::from_bytes(ctx, bytes.as_slice()).unwrap());
            }
        });

        if self.assets.is_empty() {
            return Err(RLError::AssetError("Could not find assets!".to_string()));
        }
        let (sender, receiver) = channel();
        self.sender = Some(sender);
        self.receiver = Some(receiver);
        Ok(())
    }
    /// Initializes the machines by loading the assets for all existing machines
    /// Checks if the machine has one asset if it does not change or three assets for the different states
    pub(crate) fn init_all_machines(&mut self) {
        let machine_assets: Vec<Vec<Image>> = self
            .machines
            .iter()
            .map(|m| m.id.t(Lang::De))
            .map(|name| {
                info!("Loading assets for {}", name);
                if self.assets.contains_key(&format!("{name}.png")) {
                    vec![self.assets.get(&format!("{name}.png")).unwrap().clone()]
                } else {
                    vec![
                        self.assets
                            .get(&format!("{name}_Broken.png"))
                            .unwrap()
                            .clone(),
                        self.assets
                            .get(&format!("{name}_Idle.png"))
                            .unwrap()
                            .clone(),
                        self.assets
                            .get(&format!("{name}_Running.png"))
                            .unwrap()
                            .clone(),
                    ]
                }
            })
            .collect();
        self.machines
            .iter_mut()
            .zip(machine_assets)
            .for_each(|(m, a)| {
                m.init(
                    a.as_slice(),
                    self.sender.clone().unwrap(),
                    self.screen_sender.clone().unwrap(),
                );
            });
    }

    /// Saves the active game state to a file. The boolean value "milestone" determines whether this is a milestone or an autosave.
    /// If the file already exists, it will be overwritten.
    /// # Arguments
    /// * `milestone` - Boolean value that determines whether this is a milestone save or an autosave.
    /// # Returns
    /// * `RLResult` - validates if the save was successful
    pub(crate) fn save(&self, milestone: bool) -> RLResult {
        self.save_with_root(milestone, ".")
    }

    pub(crate) fn save_with_root<P: AsRef<Path>>(&self, milestone: bool, root: P) -> RLResult {
        let save_data = serde_yaml::to_string(self)?;
        let root = root.as_ref().join("saves");

        // Create the folder if it doesn't exist
        fs::create_dir_all(&root)?;
        if milestone {
            fs::write(root.join("milestone.yaml"), save_data)?;
            info!("Saved game state as milestone");
        } else {
            fs::write(root.join("autosave.yaml"), save_data)?;
            info!("Saved game state as autosave");
        }
        Ok(())
    }

    /// Loads a game state from a file. The boolean value "milestone" determines whether this is a milestone or an autosave.
    /// If the file doesn't exist, it will return a default game state.
    /// # Arguments
    /// * `milestone` - Whether to load the milestone or the autosave
    /// # Returns
    /// * `RLResult<Gamestate>` containing the loaded game state or a default game state if the file doesn't exist.
    pub fn load(milestone: bool) -> RLResult<GameState> {
        Self::load_from_dir(milestone, Path::new("."))
    }

    fn load_from_dir<P: AsRef<Path>>(milestone: bool, root: P) -> RLResult<GameState> {
        let save_data = if milestone {
            info!("Loading milestone...");
            fs::read_to_string(root.as_ref().join("saves/milestone.yaml"))
        } else {
            info!("Loading autosave...");
            fs::read_to_string(root.as_ref().join("saves/autosave.yaml"))
        }?;
        let game_state: GameState = serde_yaml::from_str(&save_data)?;

        Ok(game_state)
    }

    /// Returns the area the player needs to stand in to interact with a machine
    /// # Returns
    /// * `Option<&mut Machine>` - The machines the player can interact with if one exists or None
    pub(crate) fn get_interactable(&mut self) -> Option<&mut Machine> {
        self.machines
            .iter_mut()
            .find(|machine| machine.is_interactable(self.player.position))
    }

    /// Returns if the player would collide with a border if they moved in the given direction
    /// # Arguments
    /// * `next_player_pos` - The direction the player wants to move
    fn border_collision_detection(next_player_pos: (usize, usize)) -> bool {
        next_player_pos.0 >= MAP_BORDER[0] // Right border
            || next_player_pos.1 >= MAP_BORDER[1] // Bottom border
            || next_player_pos.0 <= MAP_BORDER[2] // Left border
            || next_player_pos.1 <= MAP_BORDER[3] // Top border
    }
    /// Returns a boolean indicating whether the player would collide with a machine or border if they moved in the given direction
    ///
    /// # Arguments
    /// * `next_player_pos` - A tuple containing the next position of the player
    pub(crate) fn collision_detection(&self, next_player_pos: (usize, usize)) -> bool {
        self.machines
            .iter()
            .map(|area| area.hitbox)
            .any(|area| is_colliding(next_player_pos, &area))
            || Self::border_collision_detection(next_player_pos)
    }
    /// Returns the asset if it exists
    /// # Arguments
    /// * `name` - The name of the asset
    /// # Returns
    /// * `RLResult<&Image>` - The asset if it exists
    pub fn get_asset(&self, name: &str) -> RLResult<&Image> {
        self.assets.get(name).ok_or(RLError::AssetError(format!(
            "Could not find asset with name {name}"
        )))
    }
    /// Checks if the milestone is reached which means the vec of repaired machines
    /// contain the vec of machines needed to reach the next milestone.
    /// # Arguments
    /// * `milestone_machines` - A vec of machines needed to reach the next milestone
    pub fn check_on_milestone_machines(&mut self, milestone_machines: &[ObjectId]) -> bool {
        let running_machine = self
            .machines
            .iter()
            .filter(|m| m.state != Broken)
            .map(|m| m.id)
            .collect::<Vec<_>>();

        if milestone_machines
            .iter()
            .all(|machine| running_machine.contains(machine))
        {
            return true;
        }
        false
    }
    fn increase_milestone(&mut self) -> RLResult {
        self.player.milestone += 1;
        info!("Player reached milestone {}", self.player.milestone);
        self.save(true)?;
        Ok(())
    }
    /// Decides what happens if a certain milestone is reached
    /// divided into 3 milestones
    fn get_current_milestone(&mut self) -> RLResult {
        let lng = self.lang();

        match self.player.milestone {
            0 => {
                self.player.resources_change.oxygen = -1;
                self.player.resources_change.energy = -1;
                self.player.last_damage = 0;
                self.increase_milestone()?;
            }
            1 => {
                if self.check_on_milestone_machines(&[
                    ObjectId::OxygenGenerator,
                    ObjectId::PowerGenerator,
                ]) {
                    self.increase_milestone()?;
                }
            }
            3 => {
                info!("Player won the Game");
                self.player.milestone += 1;
                let cloned_sender = self.get_screen_sender()?.clone();
                self.get_screen_sender()?
                    .send(StackCommand::Screen(ScreenCommand::Push(Box::new(
                        InfoScreen::new_winning_screen(cloned_sender, lng),
                    ))))?;
            }
            _ => {}
        }
        Ok(())
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
        if ctx.time.check_update_time(DESIRED_FPS) {
            self.tick()?;
            self.move_player(ctx)?;
            Event::update_events(ctx, self)?;
        }
        Ok(())
    }
    /// Draws the game state to the screen.
    /// Draws the background, the player, the machines, the resources,
    /// the inventory and the and the handbook.
    /// # Returns
    /// `RLResult` validates the success of the drawing process
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
        self.draw_machines(&mut canvas, scale, ctx)?;
        self.draw_items(&mut canvas, ctx);
        if !self.handbook_invisible {
            self.open_handbook(&mut canvas, ctx);
        }
        #[cfg(debug_assertions)]
        {
            let fps = graphics::Text::new(format!("FPS: {}", ctx.time.fps()));
            draw!(canvas, &fps, Vec2::new(1400.0, 0.0), scale);
            let milestone = graphics::Text::new(format!("Milestone: {}", self.player.milestone));
            draw!(canvas, &milestone, Vec2::new(1400.0, 20.0), scale);
            let events = graphics::Text::new(format!("Events: {:?}", self.events));
            draw!(canvas, &events, Vec2::new(1400.0, 40.0), scale);
            let last_damage =
                graphics::Text::new(format!("Last Damage: {}", self.player.last_damage));
            draw!(canvas, &last_damage, Vec2::new(1400.0, 60.0), scale);
            let oxygen_cr = graphics::Text::new(format!(
                "Oxygen CR: {}",
                self.player.resources_change.oxygen
            ));
            draw!(canvas, &oxygen_cr, Vec2::new(1400.0, 80.0), scale);
            let energy_cr = graphics::Text::new(format!(
                "Energy CR: {}",
                self.player.resources_change.energy
            ));
            draw!(canvas, &energy_cr, Vec2::new(1400.0, 100.0), scale);
            let life_cr =
                graphics::Text::new(format!("Life CR: {}", self.player.resources_change.life));
            draw!(canvas, &life_cr, Vec2::new(1400.0, 120.0), scale);
        }
        self.draw_time(&mut canvas, scale);
        canvas.finish(ctx)?;
        Ok(())
    }
    /// Sets the screen sender to the given sender.
    /// # Arguments
    /// `sender` - The sender that is assigned to the screen sender
    fn set_sender(&mut self, sender: Sender<StackCommand>) {
        self.screen_sender = Some(sender);
        self.init_all_machines();
    }

    fn lang(&self) -> Lang {
        self.lng
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gamestate() {
        let _gamestate = GameState::new_with_lang(Lang::De);
    }

    #[test]
    fn test_save_autosave() {
        let tmp = tempdir::TempDir::new("test_save_autosave")
            .unwrap()
            .path()
            .to_path_buf();
        let gamestate = GameState::new_with_lang(Lang::De);
        gamestate.save_with_root(false, tmp).unwrap();
    }

    #[test]
    fn test_save_milestone() {
        let tmp = tempdir::TempDir::new("test_save_milestone")
            .unwrap()
            .path()
            .to_path_buf();
        let gamestate = GameState::new_with_lang(Lang::De);
        gamestate.save_with_root(true, tmp).unwrap();
    }

    #[test]
    fn test_load_autosave() {
        let tmp = tempdir::TempDir::new("test_load_autosave")
            .unwrap()
            .path()
            .to_path_buf();
        GameState::new_with_lang(Lang::De)
            .save_with_root(false, tmp.clone())
            .unwrap();
        let _gamestate_loaded = GameState::load_from_dir(false, tmp).unwrap();
    }

    #[test]
    fn test_load_milestone() {
        let tmp = tempdir::TempDir::new("test_load_milestone")
            .unwrap()
            .path()
            .to_path_buf();
        GameState::new_with_lang(Lang::De)
            .save_with_root(true, tmp.clone())
            .unwrap();
        let _gamestate_loaded = GameState::load_from_dir(true, tmp.to_path_buf()).unwrap();
    }

    #[test]
    fn test_delete_saves() {
        GameState::delete_saves().unwrap();
    }
}
