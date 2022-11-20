use crate::backend::{
    gamestate::GameState,
    screen::{Screen, StackCommand},
    utils::get_scale,
};
use crate::main_menu::button::Button;
use crate::main_menu::mainmenu::Message::{Exit, NewGame, Start};
use crate::RLResult;
use ggez::event::MouseButton;
use ggez::graphics::{Color, Text, TextFragment};
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
}

impl Default for MainMenu {
    fn default() -> Self {
        let (sender, receiver) = channel();

        let start_button = Button::new(
            "Start".to_string(),
            Start,
            sender.clone(),
            graphics::Rect::new(1322., 350., 350., 120.),
            Color::from_rgba(195, 195, 195, 255),
        );

        let new_game_button = Button::new(
            "New Game".to_string(),
            NewGame,
            sender.clone(),
            graphics::Rect::new(1322., 490., 350., 120.),
            Color::from_rgba(195, 195, 195, 255),
        );

        let exit_button = Button::new(
            "Exit".to_string(),
            Exit,
            sender.clone(),
            graphics::Rect::new(1322., 630., 350., 120.),
            Color::from_rgba(195, 195, 195, 255),
        );

        Self {
            buttons: vec![start_button, new_game_button, exit_button],
            receiver,
            sender,
        }
    }
}

impl Screen for MainMenu {
    fn update(&mut self, ctx: &mut Context) -> RLResult<StackCommand> {
        let scale = get_scale(ctx);
        //handle buttons
        if ctx.mouse.button_pressed(MouseButton::Left) {
            let current_position = ctx.mouse.position();
            dbg!(format!("Current mouse position: {current_position:?}"));
            self.buttons
                .iter_mut()
                .for_each(|btn| btn.click(current_position, scale));
        }
        if let Ok(msg) = self.receiver.try_recv() {
            match msg {
                Exit => std::process::exit(0),
                NewGame => {
                    fs::remove_file("./saves/autosave.yaml");
                    fs::remove_file("./saves/milestone.yaml");
                    Ok(StackCommand::Push(Box::new(GameState::new(ctx)?)))
                }
                Start => Ok(StackCommand::Push(Box::new({
                    let mut gamestate = GameState::load(false).unwrap_or_default();
                    gamestate.load_assets(ctx)?;
                    gamestate
                }))),
            }
        } else {
            Ok(StackCommand::None)
        }
    }

    fn draw(&self, ctx: &mut Context) -> RLResult {
        let scale = get_scale(ctx);
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));
        let background =
            graphics::Image::from_bytes(ctx, include_bytes!("../../../assets/mainmenu.png"))?;
        canvas.draw(&background, graphics::DrawParam::default().scale(scale));

        for btn in self.buttons.iter() {
            btn.draw_button(ctx, &mut canvas)?;
        }
        canvas.finish(ctx)?;

        Ok(())
    }
}
