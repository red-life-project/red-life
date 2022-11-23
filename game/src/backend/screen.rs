use crate::backend::utils::get_scale;
use crate::error::RLError;
use crate::main_menu::mainmenu::MainMenu;
use crate::{draw, RLResult};
use ggez::conf::FullscreenType::True;
use ggez::glam::vec2;
use ggez::graphics::{Color, Text};
use ggez::{event, graphics, Context};
use std::fmt::Debug;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Instant;

/// A screen is every drawable object in the game, so the main menu is a screen too
pub trait Screen: Debug {
    /// Used for updating the screen. Returns a StackCommand used to either push a new screen or pop
    /// the current one.
    fn update(&mut self, ctx: &mut Context) -> RLResult;
    /// Used for drawing the last screen in the game.
    fn draw(&self, ctx: &mut Context) -> RLResult;
    /// Set sender of the screen
    fn set_sender(&mut self, sender: Sender<StackCommand>);
}

/// A Screenstack contains multiple screens, the first one of which is the current screen
pub struct Screenstack {
    screens: Vec<Box<dyn Screen>>,
    popup: Vec<Popup>,
    receiver: Receiver<StackCommand>,
    sender: Sender<StackCommand>,
}
/// Popups are used to display ingame information/notification on screen (toplevel)
#[derive(Debug, PartialEq, Clone)]
pub struct Popup {
    color: Color,
    text: String,
    expiration: Instant,
}
impl Popup {
    pub fn nasa(text: String) -> Self {
        Self::new(RLColor::LIGHT_BLUE, text, 10)
    }
    pub fn mars(text: String) -> Self {
        Self::new(RLColor::LIGHT_GREY, text, 10)
    }
    pub fn warning(text: String) -> Self {
        Self::new(RLColor::RED, text, 10)
    }
    pub(crate) fn new(color: Color, text: String, duration: u64) -> Self {
        Self {
            color,
            text,
            expiration: Instant::now() + std::time::Duration::from_secs(duration),
        }
    }
}
impl Screenstack {
    fn draw_popups(&mut self, ctx: &mut Context) -> RLResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, None);
        for (pos, popup) in self.popup.iter().enumerate() {
            let scale = get_scale(ctx);
            let mut text = graphics::Text::new(popup.text.clone());
            text.set_scale(18.);
            let dimensions = text.measure(ctx)?;
            let x = dimensions.x;
            let y = dimensions.y;
            let rect = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(0., 0., x + 2., y + 2.),
                RLColor::LIGHT_GREY,
            )?;
            let outer = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::stroke(3.),
                graphics::Rect::new(0., 0., x + 3., y + 3.),
                RLColor::BLACK,
            )?;
            draw!(canvas, &rect, vec2(0., pos as f32 * 100.), scale);
            draw!(canvas, &outer, vec2(0., pos as f32 * 100.), scale);
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
    // Redirect the update function to the last screen and handle the returned StackCommand
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
    /// Override the quit event so we don't actually quit the game.
    fn quit_event(&mut self, ctx: &mut Context) -> RLResult<bool> {
        self.screens.last_mut().unwrap().update(ctx)?;
        Ok(true)
    }
}

impl Default for Screenstack {
    fn default() -> Self {
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
