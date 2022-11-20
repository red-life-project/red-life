use crate::backend::utils::get_scale;
use crate::main_menu::mainmenu::Message;
use crate::{draw, RLResult};
use ggez::glam::f32::Vec2;
use ggez::graphics::{Canvas, Color};
use ggez::mint::Point2;
use ggez::{graphics, Context, GameResult};
use std::sync::mpsc::Sender;

/// Clickable button
#[derive(Debug)]
pub struct Button {
    pub(crate) text: graphics::Text,
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
    pub(crate) fn draw_button(&self, ctx: &mut Context, canvas: &mut Canvas) -> RLResult {
        let mb = &mut graphics::MeshBuilder::new();
        let scale = get_scale(ctx);

        //mb.rectangle(graphics::DrawMode::fill(), btn.rect, btn.color)?;
        mb.rounded_rectangle(graphics::DrawMode::fill(), self.rect, 10.0, self.color)?;
        mb.rounded_rectangle(
            graphics::DrawMode::stroke(8.0),
            self.rect,
            10.0,
            Color::BLACK,
        )?;

        draw!(
            canvas,
            &graphics::Mesh::from_data(ctx, mb.build()),
            Vec2::new(0., 0.),
            scale
        );

        let mut text = &mut self.text.clone();
        text.set_scale(70.);

        draw!(
            canvas,
            text,
            Vec2::new(self.rect.x + 20., self.rect.y + 25.),
            scale
        );

        Ok(())
    }
}
