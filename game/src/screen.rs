use ggez::{event, Context};

use crate::error::RLError;
use crate::mainmenu::{MainMenu, Message};
use crate::RLResult;
use std::fmt::Debug;

/// A screen is every drawable object in the game, so the main menu is a screen too
pub trait Screen: Debug {
    fn update(&mut self, ctx: &mut Context) -> RLResult<StackCommand>;
    fn draw(&self, ctx: &mut Context) -> RLResult;
}

/// A Screenstack contains multiple screens, the first one of which is the current screen
pub struct Screenstack {
    screens: Vec<Box<dyn Screen>>,
}

/// The StackCommand is necessary in order to send commands back to the screenstack
/// from the screen. We can for example tell the screenstack to push the gamestate screen onto the
/// screenstack
pub enum StackCommand {
    None,
    Push(Box<dyn Screen>),
    Pop,
}

impl event::EventHandler<RLError> for Screenstack {
    fn update(&mut self, ctx: &mut Context) -> RLResult {
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
    fn draw(&mut self, ctx: &mut Context) -> RLResult {
        self.screens
            .last()
            .expect("Failed to get a screen")
            .draw(ctx)?;
        Ok(())
    }
}

impl Default for Screenstack {
    fn default() -> Self {
        Self {
            screens: vec![Box::<MainMenu<Message>>::default()],
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
