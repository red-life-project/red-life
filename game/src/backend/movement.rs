//! This file contains the movement system, which is responsible for moving the player around the map and to interact with objects.
use crate::backend::constants::MOVEMENT_SPEED;
use crate::backend::gamestate::GameState;
use crate::backend::screen::{Screen, StackCommand};
use crate::RLResult;
use ggez::winit::event::VirtualKeyCode;
use ggez::Context;
use tracing::info;

impl GameState {
    /// Handles the player movement and updates the player position
    /// Checks on every move if the next step is inside the borders of the map if not it will not move
    /// Handles escape which will pause the game and go to the main menu
    ///  # Arguments
    /// * `ctx` - The game context which is needed to get the pressed keys
    /// # Returns
    /// * `RLResult<()>` - Returns okay, if no Error occurred
    pub fn move_player(&mut self, ctx: &mut Context) -> RLResult {
        let lng = self.lang();
        if ctx.keyboard.is_key_just_pressed(VirtualKeyCode::Escape) {
            info!("Exiting...");
            self.save(false)?;
            self.get_screen_sender()?.send(StackCommand::Pop)?;
        }
        if ctx.keyboard.is_key_just_pressed(VirtualKeyCode::E) {
            info!("Interacting with Area: {:?}", self.get_interactable());
            let player_ref = &self.player.clone();
            if let Some(interactable) = self.get_interactable() {
                interactable.interact(player_ref, lng)?;
            }
        }
        if ctx.keyboard.is_key_just_pressed(VirtualKeyCode::H) {
            self.handbook_invisible = !self.handbook_invisible;
        }
        // If we are in debug mode, change the milestone by using Z
        #[cfg(debug_assertions)]
        if ctx.keyboard.is_key_just_pressed(VirtualKeyCode::Z) {
            self.player.milestone += 1;
        }
        let keys = ctx.keyboard.pressed_keys();
        for key in keys.iter() {
            match key {
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
                _ => {}
            }
        }

        Ok(())
    }
}
