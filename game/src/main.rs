mod gamestate;
mod screen;
mod error;

use std::time::Duration;
use ggez::{
    event,
    glam::*,
    graphics::{self, Color},
    Context, GameResult,
};
use crate::gamestate::GameState;

struct GuiState {
    pos_x: f32,
    circle: graphics::Mesh,
}

impl GuiState {
    fn new(ctx: &mut Context) -> GameResult<GuiState> {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            vec2(0., 0.),
            100.0,
            2.0,
            Color::WHITE,
        )?;

        Ok(GuiState { pos_x: 0.0, circle })
    }
}

impl event::EventHandler<ggez::GameError> for GuiState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        canvas.draw(&self.circle, Vec2::new(self.pos_x, 380.0));

        canvas.finish(ctx)?;

        Ok(())
    }
}

pub fn main() {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (mut ctx, event_loop) = cb.build().unwrap();
    let state = GuiState::new(&mut ctx).unwrap();
    event::run(ctx, event_loop, state);
}