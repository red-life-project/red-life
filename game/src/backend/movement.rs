use crate::backend::area::Area;
use crate::backend::gamestate::GameState;
use crate::backend::screen::StackCommand;
use crate::backend::utils::get_scale;
use crate::game_core::resources::Resources;
use crate::machines::machine::Machine;
use crate::machines::machine::State::Running;
use crate::RLResult;
use ggez::winit::event::VirtualKeyCode;
use ggez::Context;
use std::borrow::Borrow;
use tracing::info;

const MOVEMENT_SPEED: usize = 10;

impl GameState {
    pub fn move_player(&mut self, ctx: &mut Context) -> RLResult {
        if ctx.keyboard.is_key_just_pressed(VirtualKeyCode::Escape) {
            info!("Exiting...");
            self.save(false)?;
            self.screen_sender
                .as_mut()
                .unwrap()
                .send(StackCommand::Pop)?;
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
                VirtualKeyCode::E => {
                    info!("Interacting with Area: {:?}", self.get_interactable());
                    let player_ref = &self.player.clone();
                    if let Some(intractable) = self.get_interactable() {
                        intractable.interact(player_ref)
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }
}
