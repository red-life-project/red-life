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

/// Create `DeathScreen` using `deathscreen::new()` and pass reason of death from `DeathReason` enum.
/// # Example
/// ```
/// StackCommand::Push(Box::new(deathscreen::new(death_reason: DeathReason::Oxygen)?))
/// ```

/// Constants for all strings used in this screen
/// Might be moved to a separate file in the future
pub const AIR_STRING: &str = "Luft";
pub const ENERGY_STRING: &str = "Energie";
pub const AIR_AND_ENERGY_STRING: &str = "Luft und Energie";
pub const DEATH_REASON_STRING: &str = "Dein Todesgrund: ";
pub const ADDITIONAL_INFO_STRING: &str = "Bitte drücke ESC!";

/// Defines the reason for the death of the player and is used to display the reason on the screen
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DeathReason {
    Oxygen,
    Energy,
    Both,
}
impl Display for DeathReason {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DeathReason::Oxygen => write!(f, "{}", AIR_STRING),
            DeathReason::Energy => write!(f, "{}", ENERGY_STRING),
            DeathReason::Both => write!(f, "{}", AIR_AND_ENERGY_STRING),
        }
    }
}
/// `Deathscreen`, telling the user why they died. Also has a button to return to the main menu
#[derive(Debug)]
pub struct DeathScreen {
    buttons: Vec<Button>,
    death_reason: DeathReason,
    death_message: graphics::Text,
    additional_text: graphics::Text,
    sender: Sender<StackCommand>,
}

impl DeathScreen {
    /// Creates a new DeathScreen and sends a command to the `ScreenStack` when the button is pressed
    /// # Arguments
    /// * `death_reason` - The reason for the death of the player
    /// * `sender` - The sender to send the command to the `ScreenStack`
    pub fn new(death_reason: DeathReason, sender: Sender<StackCommand>) -> Self {
        info!("The player died due to a lack of : {:?}", death_reason);

        let mut death_message =
            graphics::Text::new(format!("{} {death_reason}", DEATH_REASON_STRING));
        death_message.set_scale(70.);
        let mut additional_text = graphics::Text::new(ADDITIONAL_INFO_STRING);
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
            if key == &VirtualKeyCode::Escape {
                self.sender.send(StackCommand::Push(Box::new(MainMenu::new(
                    self.sender.clone(),
                ))))?;
            };
        }
        Ok(())
    }
    /// Draws the death screen with the reason for the death of the player and a button to return to the main menu
    fn draw(&self, ctx: &mut Context) -> RLResult {
        let scale = get_scale(ctx);
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::RED);
        let background =
            graphics::Image::from_bytes(ctx, include_bytes!("../../../assets/deathscreen.png"))?;
        canvas.draw(&background, graphics::DrawParam::default().scale(scale));

        draw!(canvas, &self.death_message, Vec2::new(372., 520.), scale);

        draw!(canvas, &self.additional_text, Vec2::new(646., 720.), scale);

        canvas.finish(ctx)?;

        Ok(())
    }

    fn set_sender(&mut self, sender: Sender<StackCommand>) {
        self.sender = sender;
    }
}
