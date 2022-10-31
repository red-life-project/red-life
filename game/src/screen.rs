use ggez::{Context, event};

use crate::error::{self, RedError};
use std::fmt::Debug;
use crate::mainmenu::MainMenu;
use crate::RedResult;

/// A screen is every drawable object in the game, so the main menu is a screen too
pub trait Screen: Debug {
    fn update(&mut self, ctx: &mut Context) -> RedResult;
    fn draw(&self, ctx: &mut Context) -> RedResult;
}

/// A Screenstack contains multiple screens, the first one of which is the current screen
pub struct Screenstack {
    screens: Vec<Box<dyn Screen>>,
}
impl event::EventHandler<RedError> for Screenstack{
    fn update(&mut self, ctx: &mut Context) -> Result<(), RedError> {
        self.screens.first_mut().expect("Failed to get a screen").update(ctx)?;
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> Result<(), RedError> {
        self.screens.first().expect("Failed to get a screen").draw(ctx)?;
        Ok(())
    }
}

impl Default for Screenstack{
    fn default() -> Self {
        Self {
            screens: vec![Box::new(MainMenu::default())],
        }
    }
}