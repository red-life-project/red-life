use crate::backend::utils::get_scale;
use crate::main_menu::mainmenu::Message;
use crate::{draw, RLResult};
use ggez::glam::f32::Vec2;
use ggez::graphics::{Canvas, Color, Text, TextFragment};
use ggez::mint::Point2;
use ggez::{graphics, Context};
use std::sync::mpsc::Sender;
use tracing::info;

/// Clickable button
#[derive(Debug)]
pub struct Button {
    pub(crate) text: Text,
    pub(crate) message: Message,
    pub(crate) sender: Sender<Message>,
    pub(crate) rect: graphics::Rect,
    pub(crate) color: Color,
    pub(crate) hover_color: Color,
    pub(crate) current_color: Color,
}

impl Button {
    pub(crate) fn new(
        text: String,
        message: Message,
        sender: Sender<Message>,
        rect: graphics::Rect,
        color: Color,
        hover_color: Color,
    ) -> Self {
        info!("New Button created: text: {}, message: {:?}", text, message);
        Self {
            text: Text::new(TextFragment::new(text).color(Color::BLACK)),
            message,
            sender,
            rect,
            color,
            hover_color,
            current_color: color,
        }
    }

    // processing button interaction: click, hover
    // determines if button is clicked
    pub(crate) fn action(&mut self, ctx: &Context, scale: Vec2) {
        info!("User clicked: mouse position: {:?}", ctx.mouse.position());
        if self.in_area(ctx.mouse.position(), scale) {
            self.current_color = self.hover_color;
            if ctx.mouse.button_pressed(ggez::event::MouseButton::Left) {
                self.click();
            }
        } else {
            self.current_color = self.color;
        }
    }

    // determines if mouse is hovering over button
    fn in_area(&self, mouse_pos: Point2<f32>, scale: Vec2) -> bool {
        let mut button_rect = self.rect;
        button_rect.x *= scale.x;
        button_rect.y *= scale.y;
        button_rect.contains(mouse_pos)
    }

    // payload of button
    fn click(&mut self) {
        info!("Button clicked: message: {:?}", self.message);
        self.sender.send(self.message).unwrap();
    }

    pub(crate) fn draw_button(&self, ctx: &mut Context, canvas: &mut Canvas) -> RLResult {
        let mb = &mut graphics::MeshBuilder::new();
        let scale = get_scale(ctx);

        // Background
        mb.rounded_rectangle(
            graphics::DrawMode::fill(),
            self.rect,
            10.0,
            self.current_color,
        )?;
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

        let text = &mut self.text.clone();
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
