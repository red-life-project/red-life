use crate::main_menu::mainmenu::Message;
use ggez::glam::f32::Vec2;
use ggez::graphics::Color;
use ggez::mint::Point2;
use ggez::{graphics, Context, GameResult};
use std::sync::mpsc::Sender;

#[derive(Debug)]
pub struct Button {
    pub(crate) text: String,
    pub(crate) img: Option<graphics::Image>,
    pub(crate) message: Message,
    pub(crate) sender: Sender<Message>,
    pub(crate) rect: graphics::Rect,
    pub(crate) color: Color,
}

impl Button {
    fn pressed(&self) {
        dbg!("Pressed {:?}", self.message);
    }

    fn is_clicked(&self, mouse_pos: Point2<f32>, scale: Vec2) -> bool {
        let mut button_rect = self.rect.clone();
        button_rect.x *= scale.x;
        button_rect.y *= scale.y;
        button_rect.contains(mouse_pos)
    }
    pub(crate) fn click(&mut self, mouse_pos: Point2<f32>, scale: Vec2) {
        if self.is_clicked(mouse_pos, scale) {
            self.pressed();
            self.sender.send(self.message).unwrap();
        }
    }
}

pub fn draw_button(ctx: &mut Context, btn: &Button) -> GameResult<graphics::Mesh> {
    let mb = &mut graphics::MeshBuilder::new();

    mb.rectangle(graphics::DrawMode::fill(), btn.rect, btn.color)?;

    Ok(graphics::Mesh::from_data(ctx, mb.build()))
}
