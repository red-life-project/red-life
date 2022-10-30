use ggez::Context;
use crate::screen::Screen;
#[derive(Debug)]
struct Button{
    text: String,
    img: ggez::graphics::Image,
    // Add these:
    // message: Message,
    // sender: Sender<Message>
}
#[derive(Debug)]
struct MainMenu {
    buttons: Vec<Button>,
}

impl Screen for MainMenu {
    fn update(&mut self, ctx: &mut Context) -> crate::screen::RedResult {
        todo!()
    }

    fn draw(&self, ctx: &mut Context) -> crate::screen::RedResult {
        todo!()
    }
}