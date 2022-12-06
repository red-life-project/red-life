use crate::backend::rlcolor::RLColor;
use crate::backend::utils::*;
use crate::error::RLError;
use crate::main_menu::mainmenu::MainMenu;
use crate::{draw, RLResult};

use ggez::glam::vec2;
use ggez::graphics::Color;
use ggez::{event, graphics, Context};
use std::fmt::Debug;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Instant;
use tracing::info;

/// Screens are used to facilitate drawing menus, the game etc. to the screen. They can also send
/// back command to the `ScreenStack` to change the current screen.
pub trait Screen: Debug {
    /// Used for updating the screen. Returns a `StackCommand` used to either push a new screen or pop
    /// the current one.
    fn update(&mut self, ctx: &mut Context) -> RLResult;
    /// Used for drawing the last screen in the game.
    fn draw(&self, ctx: &mut Context) -> RLResult;
    /// Set sender of the screen
    fn set_sender(&mut self, sender: Sender<StackCommand>);
}

/// A Screenstack contains multiple `Screen`s, the last one of which is drawn to the screen and
/// updated.
pub struct Screenstack {
    screens: Vec<Box<dyn Screen>>,
    popup: Vec<Popup>,
    receiver: Receiver<StackCommand>,
    sender: Sender<StackCommand>,
}
/// Popups are used to display information sent by the game on screen (toplevel)
#[derive(Debug, PartialEq, Clone)]
pub struct Popup {
    color: Color,
    text: String,
    expiration: Instant,
}
impl Popup {
    pub fn nasa(text: String) -> Self {
        info!("New NASA popup created");
        Self::new(RLColor::LIGHT_BLUE, text, 10)
    }
    pub fn mars(text: String) -> Self {
        info!("New MARS popup created");
        Self::new(RLColor::DARK_RED, text, 10)
    }
    pub fn warning(text: String) -> Self {
        info!("New WARNING popup created");
        Self::new(RLColor::RED, text, 10)
    }
    pub fn info(text: String) -> Self {
        info!("New INFO popup created");
        Self::new(RLColor::BLACK, text, 10)
    }

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
    /// Draws a new popup at the top left of the screen with the given text and color
    /// The popup will be removed after the given duration
    fn draw_popups(&mut self, ctx: &mut Context) -> RLResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, None);
        let scale = get_scale(ctx);
        for (pos, popup) in self.popup.iter().enumerate() {
            let mut text = graphics::Text::new(popup.text.clone());
            text.set_scale(30.);
            let dimensions = text.measure(ctx)?;
            let x = dimensions.x;
            let y = dimensions.y;
            let rect = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(0., 0., x + 4., y + 4.),
                RLColor::LIGHT_GREY,
            )?;
            let outer = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::stroke(3.),
                graphics::Rect::new(0., 0., x + 4., y + 4.),
                RLColor::BLACK,
            )?;
            draw!(canvas, &rect, vec2(0., pos as f32 * (y + 4.)), scale);
            draw!(canvas, &outer, vec2(0., pos as f32 * (y + 4.)), scale);
            canvas.draw(
                &text,
                graphics::DrawParam::default()
                    .scale(scale)
                    .dest([0., pos as f32 * (y + 4.) * scale.y])
                    .color(popup.color),
            );
        }
        canvas.finish(ctx)?;
        Ok(())
    }
    /// Handles what to do with the given commands (Push, Pop, None)
    ///
    /// # Arguments
    /// * `command` - The command to handle
    ///
    /// Push: Pushes a new screen on the stack,
    /// Pop: Pops the current screen,
    /// None: Does nothing
    fn process_command(&mut self, command: StackCommand) {
        // Match the command given back by the screen
        match command {
            StackCommand::None => {}
            StackCommand::Push(mut screen) => {
                screen.set_sender(self.sender.clone());
                self.screens.push(screen);
            }
            StackCommand::Pop => {
                match self.screens.len() {
                    1 => std::process::exit(0),
                    _ => self.screens.pop(),
                };
            }
            StackCommand::Popup(popup) => self.popup.push(popup),
        }
    }
    /// Removes the expired popups
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
    Popup(Popup),
    Pop,
}

impl event::EventHandler<RLError> for Screenstack {
    /// Redirect the update function to the last screen and handle the returned StackCommand
    fn update(&mut self, ctx: &mut Context) -> RLResult {
        self.remove_popups();
        self.screens
            .last_mut()
            .expect("Failed to get a screen")
            .update(ctx)?;
        if let Ok(message) = self.receiver.try_recv() {
            self.process_command(message);
        }
        Ok(())
    }
    /// Redirect the draw command to the last screen
    fn draw(&mut self, ctx: &mut Context) -> RLResult {
        self.screens
            .last()
            .expect("Failed to get a screen")
            .draw(ctx)?;
        self.draw_popups(ctx)?;
        Ok(())
    }
    /// Overrides the quit event so we do nothing instead of quitting the game.
    fn quit_event(&mut self, _ctx: &mut Context) -> RLResult<bool> {
        Ok(true)
    }
}

impl Default for Screenstack {
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
