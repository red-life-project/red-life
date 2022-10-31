use ggez::{Context, graphics};
use crate::RedResult;
use crate::screen::Screen;
#[derive(Debug)]
struct Button{
    text: String,
    img: ggez::graphics::Image,
    // TODO: Add these:
    // message: Message,
    // sender: Sender<Message>
}
#[derive(Debug)]
pub struct MainMenu {
    buttons: Vec<Button>,
}

impl Default for MainMenu {
    fn default() -> Self {
        Self {
            buttons: vec![],
        }
    }
}

impl Screen for MainMenu {
    fn update(&mut self, ctx: &mut Context) -> RedResult {
        Ok(())
    }

    fn draw(&self, ctx: &mut Context) -> RedResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::Color::from([0.1, 0.2, 0.3, 1.0]),
        );
        let text = graphics::Text::new("Hello world");
        canvas.draw(&text, graphics::DrawParam::default());
        // TODO: Replace with ? once error.rs is implemented
        canvas.finish(ctx)?;
        Ok(())
    }
}