//TODO: refactor buttons first
use crate::main_menu::mainmenu::Button;
use crate::RedResult;
use ggez::{graphics, Context};
use std::fmt::{Debug, Formatter};
use ggez::winit::event::VirtualKeyCode;
use crate::backend::screen::{Screen, StackCommand};
use crate::backend::utils::get_scale;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DeathReason {
    Oxygen,
    Energy,
}

const DEATH_MESSAGES: [&str; 2] = [
    "You died because you ran out of oxygen!",
    "You died because you ran out of energy!",
];

#[derive(Debug)]
pub struct DeathScreen {
    buttons: Vec<Button>,
    death_reason: DeathReason,
    death_message: graphics::Text,
    additional_text: graphics::Text,
}

pub fn new(death_reason: DeathReason) -> DeathScreen {
    DeathScreen {
        buttons: vec![],
        death_reason,
        death_message: graphics::Text::new(DEATH_MESSAGES[death_reason as usize]),
        additional_text: graphics::Text::new("Press ESC to exit the game!"),
    }
}

impl Screen for DeathScreen {
    fn update(&mut self, ctx: &mut Context) -> RedResult<StackCommand> {
        let keys = ctx.keyboard.pressed_keys();
        for key in keys.iter() {
            match key {
                VirtualKeyCode::Escape => std::process::exit(0),
                _ => {}
            }
        }
        Ok(StackCommand::None)
    }

    fn draw(&self, ctx: &mut Context) -> RedResult {
        let scale = get_scale(ctx);
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::RED);

        canvas.draw(
            &self.death_message,
            graphics::DrawParam::default().dest([800., 400.]).scale(scale),
        );

        canvas.draw(
            &self.additional_text,
            graphics::DrawParam::default().dest([845., 600.]).scale(scale),
        );

        canvas.finish(ctx)?;

        Ok(())
    }
}
