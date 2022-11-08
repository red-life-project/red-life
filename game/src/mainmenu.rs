use crate::gamestate::GameState;
use crate::mainmenu::Message::{Exit, Start};
use crate::screen::{Screen, StackCommand};
use crate::utils::get_scale;
use crate::RedResult;
use ggez::event::MouseButton;
use ggez::glam::Vec2;
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
struct Button<Message: Clone> {
    text: String,
    img: Option<graphics::Image>,
    message: self::Message,
    sender: Sender<Message>,
    size: Vec2,
    pos: Vec2,
    color: Color,
}

impl Button<Message> {
    fn pressed(&self) {
        println!("Pressed {:?}", self.message);
    }

    fn is_clicked(&self, mouse_pos: Point2<f32>) -> bool {
        let x = mouse_pos.x;
        let y = mouse_pos.y;
        let x1 = self.pos.x;
        let y1 = self.pos.y;
        let x2 = self.pos.x + self.size.x;
        let y2 = self.pos.y + self.size.y;
        x > x1 && x < x2 && y > y1 && y < y2
    }

    //TODO: handle different types of buttons -> Message as type parameter
    pub fn new(
        text: String,
        img: Option<graphics::Image>,
        message: Message,
        sender: Sender<Message>,
        size: Vec2,
        pos: Vec2,
        color: Color,
    ) -> Self {
        Button {
            text,
            img,
            message,
            sender,
            size,
            pos,
            color,
        }
    }
}

#[derive(Debug)]
pub struct MainMenu<Message: Clone> {
    buttons: Vec<Button<Message>>,
    receiver: Receiver<Message>,
    sender: Sender<Message>,
}

//TODO: pos in struct for buttons

fn draw_button(ctx: &mut Context, btn: &Button<Message>) -> GameResult<graphics::Mesh> {
    let mb = &mut graphics::MeshBuilder::new();

    mb.rectangle(
        graphics::DrawMode::fill(),
        graphics::Rect::new(btn.pos[0], btn.pos[1], btn.size[0], btn.size[1]),
        btn.color,
    )?;

    Ok(graphics::Mesh::from_data(ctx, mb.build()))
}

impl<Message: Clone> Default for MainMenu<Message> {
    fn default() -> Self {
        let (sender, receiver) = channel();

        let start_button = Button {
            text: "Default Button".to_string(),
            img: None,
            message: Start,
            sender: sender.clone(),
            pos: Vec2::new(650.0, 180.0),
            size: Vec2::new(350.0, 120.0),
            color: Color::from_rgba(0, 0, 0, 50),
        };
        let exit_button = Button {
            text: "Exit".to_string(),
            img: None,
            message: Exit,
            sender: sender.clone(),
            pos: Vec2::new(650.0, 420.0),
            size: Vec2::new(350.0, 120.0),
            color: Color::from_rgba(0, 0, 0, 50),
        };

        Self {
            buttons: vec![start_button, exit_button],
            receiver,
            sender,
        }
    }
}

impl Screen for MainMenu<Message> {
    fn update(&mut self, ctx: &mut Context) -> RedResult<StackCommand> {
        //handle buttons
        if ctx.mouse.button_pressed(MouseButton::Left) {
            println!("Left mouse button pressed");
            println!("Mouse position: {:?}", ctx.mouse.position());
            let current_position = ctx.mouse.position();
            for mut btn in &mut self.buttons {
                if btn.is_clicked(current_position) {
                    btn.pressed();
                    //TODO: handle different types of buttons -> type parameter (Message) determines action?
                    if btn.message == Exit {
                        ctx.request_quit();
                    }
                    return Ok(StackCommand::Push(Box::new(GameState::default())));
                    //self.sender.send(btn.message.clone()).unwrap();
                }
            }
        }
        Ok(StackCommand::None)
    }

    fn draw(&self, ctx: &mut Context) -> RedResult {
        let scale = get_scale(ctx);
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));
        let background =
            graphics::Image::from_bytes(ctx, include_bytes!("../../assets/mainmenu.png"))?;
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
