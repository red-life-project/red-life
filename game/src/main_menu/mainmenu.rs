use crate::backend::{
    gamestate::GameState,
    screen::{Screen, StackCommand},
    utils::get_scale,
};
use crate::main_menu::mainmenu::Message::{Exit, NewGame, Start};
use crate::RLResult;
use ggez::event::MouseButton;
use ggez::graphics::Color;
use ggez::mint::Point2;
use ggez::{graphics, Context, GameResult};
use std::sync::mpsc::{channel, Receiver, Sender};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Message {
    Exit,
    NewGame,
    Start,
}

#[derive(Debug)]
struct Button<T: Clone> {
    text: String,
    img: Option<graphics::Image>,
    message: Message,
    sender: Sender<T>,
    rect: graphics::Rect,
    color: Color,
}

impl Button<Message> {
    fn pressed(&self) {
        dbg!("Pressed {:?}", self.message);
    }

    fn is_clicked(&self, mouse_pos: Point2<f32>) -> bool {
        self.rect.contains(mouse_pos)
    }
    fn click(&mut self, mouse_pos: Point2<f32>) {
        if self.is_clicked(mouse_pos) {
            self.pressed();
            self.sender.send(self.message).unwrap();
        }
    }
}

#[derive(Debug)]
pub struct MainMenu<Message: Clone> {
    buttons: Vec<Button<Message>>,
    receiver: Receiver<Message>,
    sender: Sender<Message>,
}

fn draw_button(ctx: &mut Context, btn: &Button<Message>) -> GameResult<graphics::Mesh> {
    let mb = &mut graphics::MeshBuilder::new();

    mb.rectangle(graphics::DrawMode::fill(), btn.rect, btn.color)?;

    Ok(graphics::Mesh::from_data(ctx, mb.build()))
}

impl<Message: Clone> Default for MainMenu<Message> {
    fn default() -> Self {
        let (sender, receiver) = channel();

        let start_button = Button {
            text: "Start".to_string(),
            img: None,
            message: Start,
            sender: sender.clone(),
            rect: graphics::Rect::new(650.0, 180.0, 350.0, 120.0),
            color: Color::from_rgba(0, 0, 0, 0),
        };
        let exit_button = Button {
            text: "Exit".to_string(),
            img: None,
            message: Exit,
            sender: sender.clone(),
            rect: graphics::Rect::new(650.0, 420.0, 350.0, 120.0),
            color: Color::from_rgba(0, 0, 0, 0),
        };
        let new_game_button = Button {
            text: "New Game".to_string(),
            img: None,
            message: NewGame,
            sender: sender.clone(),
            rect: graphics::Rect::new(650.0, 300.0, 350.0, 120.0),
            color: Color::from_rgba(0, 0, 0, 0),
        };

        Self {
            buttons: vec![start_button, new_game_button, exit_button],
            receiver,
            sender,
        }
    }
}

impl Screen for MainMenu<Message> {
    fn update(&mut self, ctx: &mut Context) -> RLResult<StackCommand> {
        //handle buttons
        if ctx.mouse.button_pressed(MouseButton::Left) {
            let current_position = ctx.mouse.position();
            self.buttons
                .iter_mut()
                .for_each(|btn| btn.click(current_position));
        }
        if let Ok(msg) = self.receiver.try_recv() {
            match msg {
                Exit => Ok(StackCommand::Pop),
                NewGame => Ok(StackCommand::Push(Box::new(GameState::new(ctx)?))),
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

        //draw buttons
        let mut btn_meshes = Vec::new();
        for btn in self.buttons.iter() {
            btn_meshes.push(draw_button(ctx, btn)?);
        }
        for mesh in btn_meshes {
            canvas.draw(&mesh, graphics::DrawParam::default().scale(scale));
        }
        canvas.finish(ctx)?;

        Ok(())
    }
}
