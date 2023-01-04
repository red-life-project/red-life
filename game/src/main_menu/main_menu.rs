use crate::backend::rlcolor::RLColor;
use crate::backend::{
    gamestate::GameState,
    screen::{Screen, StackCommand},
    utils::get_scale,
};
use crate::main_menu::button::Button;
use crate::RLResult;

use crate::backend::screen::{Popup, ScreenCommand};
use crate::game_core::infoscreen::InfoScreen;
use crate::languages::{button_text, resume_error_string, Lang};
use ggez::{graphics, Context};
use std::sync::mpsc::{channel, Receiver, Sender};

/// is used to define what every button does
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Message {
    Exit,
    NewGame,
    Resume,
    ChangeLanguage,
}

/// Main menu screen of the game with buttons to start a new game, load a game or exit the game.
#[derive(Debug)]
pub struct MainMenu {
    buttons: Vec<Button>,
    receiver: Receiver<Message>,
    sender: Sender<Message>,
    screen_sender: Sender<StackCommand>,
    background_image: Option<graphics::Image>,
    lng: Lang,
}

impl MainMenu {
    /// Create new `MainMenu`
    /// # Arguments
    /// * `screen_sender` - The sender of the `MainMenu` used to send messages to the `ScreenStack`.
    /// # Returns
    /// `MainMenu` - Returns a new `MainMenu`.
    pub(crate) fn new(screen_sender: Sender<StackCommand>, lng: Lang) -> MainMenu {
        let (sender, receiver) = channel();

        let mut menu = Self {
            buttons: vec![],
            receiver,
            sender,
            screen_sender,
            background_image: None,
            lng,
        };
        menu.load_buttons();
        menu
    }
}

impl MainMenu {
    fn load_buttons(&mut self) {
        let lng = self.lng;
        let sender = self.sender.clone();

        let start_button = Button::new(
            button_text(lng)[0].to_string(),
            Message::Resume,
            sender.clone(),
            graphics::Rect::new(1322., 350., 450., 120.),
            RLColor::GREY,
            RLColor::DARK_GREY,
        );

        let new_game_button = Button::new(
            button_text(lng)[1].to_string(),
            Message::NewGame,
            sender.clone(),
            graphics::Rect::new(1322., 490., 450., 120.),
            RLColor::GREY,
            RLColor::DARK_GREY,
        );

        let exit_button = Button::new(
            button_text(lng)[2].to_string(),
            Message::Exit,
            sender.clone(),
            graphics::Rect::new(1322., 630., 450., 120.),
            RLColor::GREY,
            RLColor::DARK_GREY,
        );

        let lang_button = Button::new(
            button_text(lng)[3].to_string(),
            Message::ChangeLanguage,
            sender,
            graphics::Rect::new(1322., 140. + 630., 450., 120.),
            RLColor::GREY,
            RLColor::DARK_GREY,
        );
        self.buttons = vec![start_button, new_game_button, exit_button, lang_button];
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
        let lng = self.lng;
        let scale = get_scale(ctx);
        self.buttons.iter_mut().for_each(|btn| {
            btn.action(ctx, scale);
        });
        if self.background_image.is_none() {
            self.background_image = Some(graphics::Image::from_bytes(
                ctx,
                include_bytes!("../../../assets/main_menu.png"),
            )?);
        }
        if let Ok(msg) = self.receiver.try_recv() {
            match msg {
                Message::Exit => std::process::exit(0),
                Message::NewGame => {
                    GameState::delete_saves()?;
                    let cloned_sender = self.screen_sender.clone();
                    self.screen_sender
                        .send(StackCommand::Screen(ScreenCommand::Push(Box::new(
                            InfoScreen::new_intro_screen(cloned_sender, lng),
                        ))))?;
                }
                Message::Resume => {
                    if let Ok(mut gamestate) = GameState::load(false) {
                        self.screen_sender
                            .send(StackCommand::Screen(ScreenCommand::Push(Box::new({
                                gamestate.init(ctx)?;
                                gamestate
                            }))))?;
                    } else {
                        self.screen_sender
                            .send(StackCommand::Screen(ScreenCommand::Popup(Popup::warning(
                                resume_error_string(lng).into(),
                            ))))?;
                    }
                }
                Message::ChangeLanguage => {
                    let current = self.lng;
                    match current {
                        Lang::De => {
                            self.lng = Lang::En;
                        }
                        Lang::En => {
                            self.lng = Lang::De;
                        }
                    }
                    self.load_buttons();
                }
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

    fn lang(&self) -> Lang {
        self.lng
    }
}
