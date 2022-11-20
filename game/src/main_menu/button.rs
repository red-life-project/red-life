use crate::backend::utils::get_scale;
use crate::main_menu::mainmenu::Message;
use crate::{draw, RLResult};
use ggez::glam::f32::Vec2;
use ggez::graphics::{Canvas, Color, Text, TextFragment};
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
    pub(crate) fn new(
        text: String,
        img: Option<graphics::Image>,
        message: Message,
        sender: Sender<Message>,
        rect: graphics::Rect,
        color: Color,
    ) -> Self {
        Self {
            text: Text::new(TextFragment::new(text).color(Color::BLACK)),
            img,
            message,
            sender,
            rect,
            color,
        }
    }

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

        // Background
        mb.rounded_rectangle(graphics::DrawMode::fill(), self.rect, 10.0, self.color)?;
        // Border
        mb.rounded_rectangle(
            graphics::DrawMode::stroke(8.0),
            self.rect,
            10.0,
            Color::BLACK,
        )?;

        // Draw button
        draw!(
            canvas,
            &graphics::Mesh::from_data(ctx, mb.build()),
            Vec2::new(0., 0.),
            scale
        );

        let mut text = &mut self.text.clone();
        text.set_scale(70.);

        //Draw text
        draw!(
            canvas,
            text,
            Vec2::new(self.rect.x + 20., self.rect.y + 25.),
            scale
        );

        Ok(())
    }
}
