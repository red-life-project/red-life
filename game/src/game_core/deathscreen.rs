use crate::backend::screen::{Screen, StackCommand};
use crate::backend::utils::get_scale;
use crate::main_menu::button::Button;
use crate::RLResult;
use ggez::winit::event::VirtualKeyCode;
use ggez::{graphics, Context};
use std::fmt::{Debug, Formatter};

/// Create DeathScreen using deathscreen::new() and pass reason of death from DeathReason enum.
/// # Example
/// StackCommand::Push(Box::new(deathscreen::new(death_reason: DeathReason::Oxygen)?))

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
    fn update(&mut self, ctx: &mut Context) -> RLResult<StackCommand> {
        let keys = ctx.keyboard.pressed_keys();
        for key in keys.iter() {
            return match key {
                VirtualKeyCode::Escape => Ok(StackCommand::Pop),
                _ => Ok(StackCommand::None),
            }
        }
        Ok(StackCommand::None)
    }

    fn draw(&self, ctx: &mut Context) -> RLResult {
        let scale = get_scale(ctx);
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::RED);

        canvas.draw(
            &self.death_message,
            graphics::DrawParam::default()
                .dest([800. * scale.x, 400. * scale.y])
                .scale(scale),
        );

        canvas.draw(
            &self.additional_text,
            graphics::DrawParam::default()
                .dest([845. * scale.x, 600. * scale.y])
                .scale(scale),
        );

        canvas.finish(ctx)?;

        Ok(())
    }
}
