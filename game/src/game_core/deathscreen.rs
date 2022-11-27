use crate::backend::screen::{Screen, StackCommand};
use crate::backend::utils::get_scale;
use crate::main_menu::button::Button;
use crate::main_menu::mainmenu::MainMenu;
use crate::{draw, RLResult};
use ggez::glam::Vec2;
use ggez::winit::event::VirtualKeyCode;
use ggez::{graphics, Context};
use std::fmt::{Debug, Display, Formatter};
use std::sync::mpsc::Sender;
use tracing::info;

/// Create DeathScreen using deathscreen::new() and pass reason of death from DeathReason enum.
/// # Example
/// StackCommand::Push(Box::new(deathscreen::new(death_reason: DeathReason::Oxygen)?))

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DeathReason {
    Oxygen,
    Energy,
    Both,
}
impl Display for DeathReason {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DeathReason::Oxygen => write!(f, "Luft"),
            DeathReason::Energy => write!(f, "Energie"),
            DeathReason::Both => write!(f, "Luft und Energie"),
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
    sender: Sender<StackCommand>,
}

impl DeathScreen {
    pub fn new(death_reason: DeathReason, sender: Sender<StackCommand>) -> Self {
        info!("The player died due to a lack of : {:?}", death_reason);

        let mut death_message = graphics::Text::new(format!("Dein Todesgrund: {death_reason}"));
        death_message.set_scale(70.);
        let mut additional_text = graphics::Text::new("Bitte drücke ESC!");
        additional_text.set_scale(70.);

        Self {
            buttons: vec![],
            death_reason,
            death_message,
            additional_text,
            sender,
        }
    }
}

impl Screen for DeathScreen {
    fn update(&mut self, ctx: &mut Context) -> RLResult {
        let keys = ctx.keyboard.pressed_keys();
        if let Some(key) = keys.iter().next() {
            info!(
                "The player wants to return to the main menu with: {:?}",
                key
            );
            match key {
                VirtualKeyCode::Escape => self.sender.send(StackCommand::Push(Box::new(
                    MainMenu::new(self.sender.clone()),
                )))?,
                _ => {}
            };
        }
        Ok(())
    }

    fn draw(&self, ctx: &mut Context) -> RLResult {
        let scale = get_scale(ctx);
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::RED);
        let background =
            graphics::Image::from_bytes(ctx, include_bytes!("../../../assets/deathscreen.png"))?;
        canvas.draw(&background, graphics::DrawParam::default().scale(scale));

        draw!(canvas, &self.death_message, Vec2::new(412., 520.), scale);

        draw!(canvas, &self.additional_text, Vec2::new(686., 720.), scale);

        canvas.finish(ctx)?;

        Ok(())
    }

    fn set_sender(&mut self, sender: Sender<StackCommand>) {
        self.sender = sender;
    }
}
