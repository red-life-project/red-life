use std::sync::mpsc::{channel, Receiver, Sender};
use crate::screen::{Screen, StackCommand};
use crate::RedResult;
use ggez::{graphics, Context};
use ggez::event::MouseButton;
use crate::gamestate::GameState;
use crate::utils::get_scale;

#[derive(Copy, Clone, Debug)]
pub enum Message {
    Exit,
    NewGame,
    Start
}

#[derive(Debug)]
struct Button<Message: Clone> {
    text: String,
    img: graphics::Image,
    message: Message,
    sender: Sender<Message>
}

impl Button<Message> {
    fn pressed(&mut self) {
        self.color = Color::GREEN;
    }
}


#[derive(Debug)]
pub struct MainMenu<Message: Clone> {
    buttons: Vec<Button<Message>>,
    receiver: Receiver<Message>,
    sender: Sender<Message>
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

        let default_button = Button {
            text: "Default Button".to_string(),
            img: None,
            message: self::Message::NewGame,
            sender: None,
            pos: Vec2::new(200.0, 100.0),
            size: Vec2::new(350.0, 100.0),
            color: Color::CYAN
        };

        let (sender, receiver) = channel();
        Self{
            buttons: vec![
                default_button
            ],
            receiver,
            sender
        }
    }
}

impl Screen for MainMenu<Message> {
    fn update(&mut self, ctx: &mut Context) -> RedResult<StackCommand> {
        // TODO: Replace with if buttons are clicked
        if ctx.mouse.button_pressed(MouseButton::Left) {
            println!("Left mouse button pressed");
            let current_position = ctx.mouse.position();
            for mut btn in &mut self.buttons {
                if current_position.x > btn.pos[0] && current_position.x < btn.pos[0] + btn.size[0] {
                    if current_position.y > btn.pos[1] && current_position.y < btn.pos[1] + btn.size[1] {
                        btn.pressed();
                        println!("Button clicked");
                        //self.sender.send(btn.message.clone()).unwrap();
                    }
                }
            }
            //return Ok(StackCommand::Push(Box::new(GameState::default())));
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

