//!DIESE DATEI IST ZUM TESTEN VON SANDER
use crate::backend::gamestate::GameState;
use crate::machines::machine::Machine;
use crate::{draw, RLResult};
use ggez::glam::Vec2;
use ggez::graphics::Canvas;
use ggez::Context;
use tracing::info;

impl GameState {
    pub fn create_machine(&mut self) -> RLResult {
        info!("Generating all Machines");
        let new_ms = Machine::quick(self)?;
        self.areas.push(Box::new(new_ms));
        Ok(())
    }

    pub fn draw_machines(&self, canvas: &mut Canvas, scale: Vec2, ctx: &mut Context) {
        for area in &self.areas {
            let machine = area.get_graphic();
            let pos = Vec2 {
                x: area.get_collision_area().x,
                y: area.get_collision_area().y,
            };
            draw!(canvas, &machine, pos, scale);
        }
    }
}
