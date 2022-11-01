use crate::screen::Screen;
use crate::RedResult;
use ggez::{graphics, Context};
use crate::utils::get_scale;

#[derive(Debug)]
struct Button {
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
        Self { buttons: vec![] }
    }
}

impl Screen for MainMenu {
    fn update(&mut self, ctx: &mut Context) -> RedResult {
        Ok(())
    }

    fn draw(&self, ctx: &mut Context) -> RedResult {
        let scale = get_scale(ctx);
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));
        let background =
            graphics::Image::from_bytes(ctx, include_bytes!("../../assets/mainmenu.png"))?;
        canvas.draw(&background, graphics::DrawParam::default().scale(scale));
        canvas.finish(ctx)?;
        Ok(())
    }
}
