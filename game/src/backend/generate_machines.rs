use ggez::{Context, graphics};
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, DrawMode};
use crate::backend::gamestate::GameState;
use crate::{draw, RLResult};
use crate::machines::machine::Mashine;

///DIESE DATEI IST ZUM TESTEN VON SANDER

impl GameState{
    pub fn create_machien(&mut self){

        let new_ms = Mashine::test_mashine(self);
        self.areas.push(Box::new(new_ms));
    }

    pub fn draw_mashiens(&self, canvas: &mut Canvas, scale: Vec2, ctx: &mut Context) -> RLResult {
        for area in &self.areas {
/*
            let mesh = graphics::Mesh::new_rounded_rectangle(
                ctx,
                DrawMode::fill(),
                rect,
                0.0,
                Color::from(COLORS[i]),
            )?;
*/
            let maschien = area.get_graphic();
            let pos = Vec2{ x: area.get_collision_area().x, y: area.get_collision_area().y};
            draw!(
            canvas,
            maschien,
            pos,
            scale
        );
        }
    Ok(())
    }
}
