use crate::backend::rlcolor::RLColor;
use crate::backend::{
    gamestate::GameState,
    screen::{Screen, StackCommand},
    utils::get_scale,
};
use crate::main_menu::button::Button;
use crate::main_menu::mainmenu::Message::{Exit, NewGame, Start};
use crate::RLResult;

use crate::game_core::infoscreen::InfoScreen;
use crate::languages::german::BUTTON_TEXT;
use ggez::{graphics, Context};
use std::sync::mpsc::{channel, Receiver, Sender};

/// is used to define what every button does
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Message {
    Exit,
    NewGame,
    Start,
}

/// Main menu screen of the game with buttons to start a new game, load a game or exit the game.
#[derive(Debug)]
pub struct MainMenu {
    buttons: Vec<Button>,
    receiver: Receiver<Message>,
    _sender: Sender<Message>,
    screen_sender: Sender<StackCommand>,
    background_image: Option<graphics::Image>,
}

impl MainMenu {
    /// Create new `MainMenu`
    /// # Arguments
    /// * `screen_sender` - The sender of the `MainMenu` used to send messages to the `ScreenStack`.
    /// # Returns
    /// `MainMenu` - Returns a new `MainMenu`.
    pub(crate) fn new(screen_sender: Sender<StackCommand>) -> MainMenu {
        let (sender, receiver) = channel();

        let start_button = Button::new(
            BUTTON_TEXT[0].to_string(),
            Start,
            sender.clone(),
            graphics::Rect::new(1322., 350., 450., 120.),
            RLColor::GREY,
            RLColor::DARK_GREY,
        );

        let new_game_button = Button::new(
            BUTTON_TEXT[1].to_string(),
            NewGame,
            sender.clone(),
            graphics::Rect::new(1322., 490., 450., 120.),
            RLColor::GREY,
            RLColor::DARK_GREY,
        );

        let exit_button = Button::new(
            BUTTON_TEXT[2].to_string(),
            Exit,
            sender.clone(),
            graphics::Rect::new(1322., 630., 450., 120.),
            RLColor::GREY,
            RLColor::DARK_GREY,
        );

        Self {
            buttons: vec![start_button, new_game_button, exit_button],
            receiver,
            _sender: sender,
            screen_sender,
            background_image: None,
        }
    }
}

/// Implement the `Screen` trait for `MainMenu`
impl Screen for MainMenu {
    /// Updates the screen every tick
    /// # Arguments
    /// * `ctx` - The ggez context
    /// # Returns
    /// `RLResult` - Returns an `RLResult`.
    fn update(&mut self, ctx: &mut Context) -> RLResult {

        let scale = get_scale(ctx);
        self.buttons.iter_mut().for_each(|btn| {
            btn.action(ctx, scale);
        });
        if self.background_image.is_none() {
            self.background_image = Some(graphics::Image::from_bytes(
                ctx,
                include_bytes!("../../../assets/mainmenu.png"),
            )?);
        }
        if let Ok(msg) = self.receiver.try_recv() {
            match msg {
                Exit => std::process::exit(0),
                NewGame => {
                    GameState::delete_saves()?;
                    let cloned_sender = self.screen_sender.clone();
                    self.screen_sender.send(StackCommand::Push(Box::new(
                        InfoScreen::new_introscreen(cloned_sender),
                    )))?;
                }
                Start => self.screen_sender.send(StackCommand::Push(Box::new({
                    let mut gamestate = GameState::load(false).unwrap_or_default();
                    gamestate.init(ctx)?;
                    gamestate
                })))?,
            }
        }
        Ok(())
    }
    /// Draws the main menu and all its buttons.
    /// # Arguments
    /// * `ctx` - The ggez context
    /// # Returns
    /// `RLResult` - Returns an `RLResult`.
    fn draw(&self, ctx: &mut Context) -> RLResult {
        let scale = get_scale(ctx);
        let mut canvas = graphics::Canvas::from_frame(ctx, RLColor::DARK_BLUE);
        if let Some(background) = &self.background_image {
            canvas.draw(background, graphics::DrawParam::default().scale(scale));
        }
        for btn in &self.buttons {
            btn.draw_button(ctx, &mut canvas)?;
        }
        canvas.finish(ctx)?;

        Ok(())
    }

    fn set_sender(&mut self, _sender: Sender<StackCommand>) {}
}
