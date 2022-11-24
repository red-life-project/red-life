use crate::backend::rlcolor::RLColor;
use crate::backend::{
    gamestate::GameState,
    screen::{Screen, StackCommand},
    utils::get_scale,
};
use crate::main_menu::button::Button;
use crate::main_menu::mainmenu::Message::{Exit, NewGame, Start};
use crate::RLResult;
use ggez::{graphics, Context};
use std::fs;
use std::sync::mpsc::{channel, Receiver, Sender};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Message {
    Exit,
    NewGame,
    Start,
}

#[derive(Debug)]
pub struct MainMenu {
    buttons: Vec<Button>,
    receiver: Receiver<Message>,
    sender: Sender<Message>,
    screen_sender: Sender<StackCommand>,
}

impl Screen for MainMenu {
    fn update(&mut self, ctx: &mut Context) -> RLResult {
        let scale = get_scale(ctx);
        self.buttons.iter_mut().for_each(|btn| {
            btn.action(ctx, scale);
        });

        if let Ok(msg) = self.receiver.try_recv() {
            match msg {
                Exit => std::process::exit(0),
                NewGame => {
                    fs::remove_file("./saves/autosave.yaml");
                    fs::remove_file("./saves/milestone.yaml");
                    self.screen_sender
                        .send(StackCommand::Push(Box::new(GameState::new(ctx)?)))?;
                }
                Start => self.screen_sender.send(StackCommand::Push(Box::new({
                    let mut gamestate = GameState::load(false).unwrap_or_default();
                    gamestate.load_assets(ctx)?;
                    gamestate
                })))?,
            }
        }
        Ok(())
    }

    fn draw(&self, ctx: &mut Context) -> RLResult {
        let scale = get_scale(ctx);
        let mut canvas = graphics::Canvas::from_frame(ctx, RLColor::DARK_BLUE);
        let background =
            graphics::Image::from_bytes(ctx, include_bytes!("../../../assets/mainmenu.png"))?;
        canvas.draw(&background, graphics::DrawParam::default().scale(scale));

        for btn in self.buttons.iter() {
            btn.draw_button(ctx, &mut canvas)?;
        }
        canvas.finish(ctx)?;

        Ok(())
    }

    fn set_sender(&mut self, sender: Sender<StackCommand>) {}
}

impl MainMenu {
    pub(crate) fn new(screen_sender: Sender<StackCommand>) -> MainMenu {
        let (sender, receiver) = channel();

        let start_button = Button::new(
            "Start".to_string(),
            Start,
            sender.clone(),
            graphics::Rect::new(1322., 350., 450., 120.),
            RLColor::GREY,
            RLColor::DARK_GREY,
        );

        let new_game_button = Button::new(
            "Neues Spiel".to_string(),
            NewGame,
            sender.clone(),
            graphics::Rect::new(1322., 490., 450., 120.),
            RLColor::GREY,
            RLColor::DARK_GREY,
        );

        let exit_button = Button::new(
            "Beenden".to_string(),
            Exit,
            sender.clone(),
            graphics::Rect::new(1322., 630., 450., 120.),
            RLColor::GREY,
            RLColor::DARK_GREY,
        );

        Self {
            buttons: vec![start_button, new_game_button, exit_button],
            receiver,
            sender,
            screen_sender,
        }
    }
}
