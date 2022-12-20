use crate::backend::utils::{get_draw_params, get_scale};
use crate::main_menu::mainmenu::Message;
use crate::{draw, RLResult};
use good_web_game::event::GraphicsContext;
use good_web_game::graphics::{Canvas, Color, Drawable, Text, TextFragment, Vector2};
use good_web_game::input::mouse::{button_pressed, position};
use good_web_game::mint::Point2;
use good_web_game::{graphics, Context};
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
    pub(crate) fn action(&mut self, ctx: &Context, scale: Vector2) {
        if self.in_area(position(ctx), scale) {
            self.current_color = self.hover_color;
            if button_pressed(ctx, good_web_game::event::MouseButton::Left) {
                info!("User clicked: mouse position: {:?}", position(ctx));
                self.click();
            }
        } else {
            self.current_color = self.color;
        }
    }

    // determines if mouse is hovering over button
    fn in_area(&self, mouse_pos: Point2<f32>, scale: Vector2) -> bool {
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

    pub(crate) fn draw_button(&self, ctx: &mut Context, gfx: &mut GraphicsContext) -> RLResult {
        let mb = &mut graphics::MeshBuilder::new();
        let scale = get_scale(gfx);

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
        let mesh = mb.build(ctx, gfx)?;
        // Draw button
        graphics::draw(
            ctx,
            gfx,
            &mesh,
            get_draw_params(
                Some(Vector2 {
                    x: self.rect.x + self.rect.w / 2.0,
                    y: self.rect.y + self.rect.h / 2.0,
                }),
                scale,
                None,
            ),
        )?;

        let text = &mut self.text.clone();
        //Draw text
        graphics::draw(
            ctx,
            gfx,
            text,
            get_draw_params(
                Some(Vector2 {
                    x: self.rect.x + self.rect.w / 2.0,
                    y: self.rect.y + self.rect.h / 2.0,
                }),
                scale,
                None,
            ),
        )?;

        Ok(())
    }
}
