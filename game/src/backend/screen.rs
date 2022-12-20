//! Contains the screen system, which is responsible for managing the different screens of the game.
use crate::backend::rlcolor::RLColor;
use crate::backend::utils::{get_draw_params, get_scale};
use crate::error::RLError;
use crate::main_menu::mainmenu::MainMenu;
use crate::{draw, RLResult};

use good_web_game::event::{GraphicsContext, KeyCode, KeyMods};
use good_web_game::graphics::{Color, Vector2};
use good_web_game::{event, graphics, Context};
use std::fmt::Debug;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Instant;
use tracing::info;

/// Screens are used to facilitate drawing menus, the game etc. to the screen. They can also send
/// back command to the `ScreenStack` to change the current screen.
pub trait Screen: Debug {
    /// Used for updating the screen.
    /// # Arguments
    /// * `ctx` - The ggez context
    /// # Returns
    /// `RLResult` - Returns an `RlResult`.
    fn update(&mut self, ctx: &mut Context, _gfx: &mut GraphicsContext) -> RLResult;
    /// Used for drawing the screen.
    /// # Arguments
    /// * `ctx` - The ggez context
    /// # Returns
    /// `RLResult` - Returns an `RlResult`.
    fn draw(&self, ctx: &mut Context, _gfx: &mut GraphicsContext) -> RLResult;
    /// Set the sender of the screen.
    /// # Arguments
    /// * `sender` - The sender of the screen.
    fn set_sender(&mut self, sender: Sender<StackCommand>);
}

/// A Screenstack contains multiple `Screen`s and `Popup`s, the last one of which is drawn to the screen and
/// updated.
pub struct Screenstack {
    screens: Vec<Box<dyn Screen>>,
    popup: Vec<Popup>,
    receiver: Receiver<StackCommand>,
    sender: Sender<StackCommand>,
}

/// Popups are used to display information sent by the game on screen (toplevel)
/// A Popup is made up of a color, a text and a expiration.
#[derive(Debug, PartialEq, Clone)]
pub struct Popup {
    color: Color,
    text: String,
    expiration: Instant,
}
impl Popup {
    /// Creates a new `Popup` from the nasa template.
    /// # Arguments
    /// * `text` - The text of the `Popup`.
    /// # Returns
    /// `Popup` - Returns a new `Popup`.
    pub fn nasa(text: String) -> Self {
        info!("New NASA popup created");
        Self::new(RLColor::LIGHT_BLUE, text, 10)
    }
    /// Creates a new `Popup` from the mars template.
    /// # Arguments
    /// * `text` - The text of the `Popup`.
    /// # Returns
    /// `Popup` - Returns a new `Popup`.
    pub fn mars(text: String) -> Self {
        info!("New MARS popup created");
        Self::new(RLColor::DARK_RED, text, 10)
    }
    /// Creates a new `popup` from the warning template.
    /// # Arguments
    /// * `text` - The text of the popup.
    /// # Returns
    /// `Popup` - Returns a new `Popup`.
    pub fn warning(text: String) -> Self {
        info!("New WARNING popup created");
        Self::new(RLColor::RED, text, 10)
    }
    /// Creates a new `Popup` from the info template.
    /// # Arguments
    /// * `text` - The text of the `Popup`.
    /// # Returns
    /// `Popup` - Returns a new `Popup`.
    pub fn info(text: String) -> Self {
        info!("New INFO popup created");
        Self::new(RLColor::BLACK, text, 10)
    }
    /// Creates a new `Popup` from a color, text and a duration.
    /// # Arguments
    /// * `color` - The color of the `Popup`.
    /// * `text` - The text of the `Popup`.
    /// * `duration` - The duration of the `Popup`.
    /// # Returns
    /// `Popup` - Returns a new `Popup`.
    pub(crate) fn new(color: Color, text: String, duration: u64) -> Self {
        info!("New popup created: text: {}, duration: {}", text, duration);
        Self {
            color,
            text,
            expiration: Instant::now() + std::time::Duration::from_secs(duration),
        }
    }
}
impl Screenstack {
    /// Draws all `Popups` at the top left of the screen with their given text and color
    /// The popups will be removed after the given duration
    /// # Arguments
    /// * `ctx` - The ggez game context
    /// # Returns
    /// `RLResult` - Returns an `RlResult`.
    fn draw_popups(&mut self, ctx: &mut Context, gfx: &mut GraphicsContext) -> RLResult {
        let scale = get_scale(gfx);
        let mut new_y = 0.0;
        for popup in &self.popup {
            let mut text = graphics::Text::new(popup.text.clone());
            // TODO text.set_scale(25.);
            let dimensions = text.dimensions(ctx);
            let x = dimensions.x;
            let y = dimensions.y;
            let rect = graphics::Mesh::new_rectangle(
                ctx,
                gfx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(0., 0., x, y),
                RLColor::LIGHT_GREY,
            )?;
            let outer = graphics::Mesh::new_rectangle(
                ctx,
                gfx,
                graphics::DrawMode::stroke(3.),
                graphics::Rect::new(0., 0., x, y),
                RLColor::BLACK,
            )?;
            graphics::draw(
                ctx,
                gfx,
                &rect,
                get_draw_params(Some(Vector2::new(0., new_y)), scale, None),
            )?;
            graphics::draw(
                ctx,
                gfx,
                &outer,
                get_draw_params(Some(Vector2::new(0., new_y)), scale, None),
            )?;
            graphics::draw(
                ctx,
                gfx,
                &text,
                get_draw_params(Some(Vector2::new(0., new_y)), scale, None),
            )?;
            new_y += y;
        }
        Ok(())
    }
    /// Handles what to do with the given commands.
    /// Possible commands are:
    /// `Push`: Pushes a new screen on the stack,
    /// `Pop`: Pops the current screen,
    /// `Popup`: Adds a new popup to the stack
    /// # Arguments
    /// * `command` - The command to handle
    fn process_command(&mut self, command: StackCommand) {
        // Match the command given back by the screen
        match command {
            StackCommand::Push(mut screen) => {
                screen.set_sender(self.sender.clone());
                self.screens.push(screen);
            }
            StackCommand::Pop => {
                if self.screens.len() == 1 {
                    std::process::exit(0)
                } else {
                    // Clear our popups in order to not display them outside of the Gamestate
                    self.popup.clear();
                    self.screens.pop();
                };
            }
            StackCommand::Popup(popup) => self.popup.push(popup),
        }
    }
    /// Removes the expired `Popup`s
    fn remove_popups(&mut self) {
        self.popup.retain(|popup| popup.expiration > Instant::now());
    }
}

/// The `StackCommand` is necessary in order to send commands back to the `Screenstack`
/// from the `Screen`.
/// # Examples
/// We can tell the `Screenstack` to push the `Gamestate` screen onto the
/// `Screenstack`
pub enum StackCommand {
    Push(Box<dyn Screen>),
    Popup(Popup),
    Pop,
}

impl event::EventHandler<RLError> for Screenstack {
    /// Redirect the update function to the last screen and handle the returned `StackCommand`
    /// # Arguments
    /// * `ctx` - The ggez game context
    /// # Returns
    /// `RLResult` - Returns an `RlResult`
    fn update(&mut self, ctx: &mut Context, gfx: &mut GraphicsContext) -> RLResult {
        self.remove_popups();
        self.screens
            .last_mut()
            .expect("Failed to get a screen")
            .update(ctx, gfx)?;
        if let Ok(message) = self.receiver.try_recv() {
            self.process_command(message);
        }
        Ok(())
    }
    /// Redirect the draw command to the last screen.
    /// # Arguments
    /// * `ctx` - The ggez game context
    /// # Returns
    /// `RLResult` - Returns an `RlResult`
    fn draw(&mut self, ctx: &mut Context, gfx: &mut GraphicsContext) -> RLResult {
        self.screens
            .last()
            .expect("Failed to get a screen")
            .draw(ctx, gfx)?;
        self.draw_popups(ctx, gfx)?;
        Ok(())
    }
}

impl Default for Screenstack {
    /// Creates a new `Screenstack` with a `MainMenu` screen.
    /// # Returns
    /// `Screenstack` - Returns a new `Screenstack`.
    fn default() -> Self {
        info!("Default Screenstack created");
        let (sender, receiver) = channel();
        Self {
            screens: vec![Box::new(MainMenu::new(sender.clone()))],
            popup: vec![],
            receiver,
            sender,
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
