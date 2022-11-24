use crate::backend::screen::{Screen, StackCommand};
use crate::backend::utils::get_scale;
use crate::main_menu::button::Button;
use crate::main_menu::mainmenu::MainMenu;
use crate::{draw, RLResult};
use ggez::glam::Vec2;
use ggez::winit::event::VirtualKeyCode;
use ggez::{graphics, Context};
use std::fmt::{Debug, Display, Formatter};

/// Create DeathScreen using deathscreen::new() and pass reason of death from DeathReason enum.
/// # Example
/// StackCommand::Push(Box::new(deathscreen::new(death_reason: DeathReason::Oxygen)?))

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DeathReason {
    Oxygen,
    Energy,
}
impl Display for DeathReason {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DeathReason::Oxygen => write!(f, "Luft"),
            DeathReason::Energy => write!(f, "Energie"),
        }
    }
}
/// Deathscreen, telling the user why they died.
#[derive(Debug)]
pub struct DeathScreen {
    buttons: Vec<Button>,
    death_reason: DeathReason,
    death_message: graphics::Text,
    additional_text: graphics::Text,
}

impl DeathScreen {
    pub fn new(death_reason: DeathReason) -> Self {
        Self {
            buttons: vec![],
            death_reason,
            death_message: graphics::Text::new(format!("Dein Todesgrund: {death_reason}")),
            additional_text: graphics::Text::new("Bitte drücke ESC!"),
        }
    }
}

impl Screen for DeathScreen {
    fn update(&mut self, ctx: &mut Context) -> RLResult<StackCommand> {
        let keys = ctx.keyboard.pressed_keys();
        if let Some(key) = keys.iter().next() {
            return match key {
                VirtualKeyCode::Escape => Ok(StackCommand::Push(Box::new(MainMenu::default()))),
                _ => Ok(StackCommand::None),
            };
        }
        Ok(StackCommand::None)
    }

    fn draw(&self, ctx: &mut Context) -> RLResult {
        let scale = get_scale(ctx);
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::RED);

        draw!(
            canvas,
            &self.death_message,
            Vec2::new(400., 200.),
            2. * scale
        );

        draw!(
            canvas,
            &self.additional_text,
            Vec2::new(422.5, 300.),
            2. * scale
        );

        canvas.finish(ctx)?;

        Ok(())
    }
}
