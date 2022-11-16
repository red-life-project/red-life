use crate::backend::utils::get_scale;
use crate::error::RLError;
use crate::main_menu::mainmenu::{MainMenu, Message};
use crate::{draw, RLResult};
use ggez::graphics::Color;
use ggez::{event, graphics, Context};
use std::fmt::Debug;
use std::time::Instant;

/// A screen is every drawable object in the game, so the main menu is a screen too
pub trait Screen: Debug {
    /// Used for updating the screen. Returns a StackCommand used to either push a new screen or pop
    /// the current one.
    fn update(&mut self, ctx: &mut Context) -> RLResult<StackCommand>;
    /// Used for drawing the last screen in the game.
    fn draw(&self, ctx: &mut Context) -> RLResult;
}

/// A Screenstack contains multiple screens, the first one of which is the current screen
pub struct Screenstack {
    screens: Vec<Box<dyn Screen>>,
    popup: Vec<Popup>,
}
/// Popups are used to display ingame information/notification on screen (toplevel)
#[derive(Debug, PartialEq, Clone)]
pub struct Popup {
    color: Color,
    text: String,
    expiration: Instant,
}
impl Popup {
    pub fn new(color: Color, text: String, duration: u64) -> Self {
        Self {
            color,
            text,
            expiration: Instant::now() + std::time::Duration::from_secs(duration),
        }
    }
}
impl Screenstack {
    fn draw_popup(&mut self, ctx: &mut Context) -> RLResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, None);
        for (pos, popup) in self.popup.iter().enumerate() {
            let scale = get_scale(ctx);
            let text = graphics::Text::new(popup.text.clone());
            canvas.draw(
                &text,
                graphics::DrawParam::default()
                    .scale(scale)
                    .dest([0., pos as f32 * 100. * scale.y])
                    .color(popup.color),
            );
        }
        canvas.finish(ctx)?;
        Ok(())
    }
    fn remove_popups(&mut self) {
        self.popup.retain(|popup| popup.expiration > Instant::now());
    }
}
/// The `StackCommand` is necessary in order to send commands back to the `Screenstack`
/// from the `Screen`. We can for example tell the screenstack to push the `Gamestate` screen onto the
/// `Screenstack`
pub enum StackCommand {
    None,
    Push(Box<dyn Screen>),
    Pop,
}

impl event::EventHandler<RLError> for Screenstack {
    // Redirect the update function to the last screen and handle the returned StackCommand
    fn update(&mut self, ctx: &mut Context) -> RLResult {
        self.remove_popups();
        let command = self
            .screens
            .last_mut()
            .expect("Failed to get a screen")
            .update(ctx)?;
        // Match the command given back by the screen
        match command {
            StackCommand::None => {}
            StackCommand::Push(screen) => self.screens.push(screen),
            StackCommand::Pop => {
                match self.screens.len() {
                    1 => std::process::exit(0),
                    _ => self.screens.pop(),
                };
            }
        }
        Ok(())
    }
    /// Override the quit event so we don't actually quit the game.
    fn quit_event(&mut self, ctx: &mut Context) -> RLResult<bool> {
        self.screens.last_mut().unwrap().update(ctx)?;
        Ok(true)
    }
    /// Redirect the draw command to the last screen
    fn draw(&mut self, ctx: &mut Context) -> RLResult {
        self.screens
            .last()
            .expect("Failed to get a screen")
            .draw(ctx)?;
        self.draw_popup(ctx)?;
        Ok(())
    }
}

impl Default for Screenstack {
    fn default() -> Self {
        Self {
            screens: vec![Box::<MainMenu>::default()],
            popup: vec![],
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_screenstack() {
        let screenstack = Screenstack::default();
        assert_eq!(1, screenstack.screens.len());
    }
}
