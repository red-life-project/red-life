use crate::backend::gamestate::GameState;
use crate::backend::screen::StackCommand;
use crate::backend::utils::get_scale;
use crate::RLResult;
use ggez::winit::event::VirtualKeyCode;
use ggez::Context;

const MOVEMENT_SPEED: usize = 5;

impl GameState {
    pub fn move_player(&mut self, ctx: &mut Context) -> RLResult<StackCommand> {
        let scale = get_scale(ctx);
        let keys = ctx.keyboard.pressed_keys();
        for key in keys.iter() {
            match key {
                VirtualKeyCode::Escape => {
                    self.save(false)?;
                    return Ok(StackCommand::Pop);
                }
                VirtualKeyCode::W => {
                    if !self.collision_detection(
                        (
                            self.player.position.0,
                            self.player.position.1.saturating_sub(MOVEMENT_SPEED),
                        ),
                        scale,
                    ) {
                        self.player.position.1 =
                            self.player.position.1.saturating_sub(MOVEMENT_SPEED);
                    }
                }
                VirtualKeyCode::A => {
                    if !self.collision_detection(
                        (
                            self.player.position.0.saturating_sub(MOVEMENT_SPEED),
                            self.player.position.1,
                        ),
                        scale,
                    ) {
                        self.player.position.0 =
                            self.player.position.0.saturating_sub(MOVEMENT_SPEED);
                    }
                }
                VirtualKeyCode::S => {
                    if !self.collision_detection(
                        (
                            self.player.position.0,
                            self.player.position.1.saturating_add(MOVEMENT_SPEED),
                        ),
                        scale,
                    ) {
                        self.player.position.1 =
                            self.player.position.1.saturating_add(MOVEMENT_SPEED);
                    }
                }
                VirtualKeyCode::D => {
                    if !self.collision_detection(
                        (
                            self.player.position.0.saturating_add(MOVEMENT_SPEED),
                            self.player.position.1,
                        ),
                        scale,
                    ) {
                        self.player.position.0 =
                            self.player.position.0.saturating_add(MOVEMENT_SPEED);
                    }
                }
                key => {
                    dbg!("{:?}", key);
                }
            }
        }

        Ok(StackCommand::None)
    }
}
