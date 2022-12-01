//!DIESE DATEI IST ZUM TESTEN VON SANDER
use crate::backend::gamestate::{GameCommand, GameState};
use std::sync::mpsc::Sender;

use crate::machines::machine::Machine;

use crate::backend::constants::gen_all_machines;
use crate::{draw, RLResult};
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Mesh};
use ggez::Context;
use tracing::info;

impl GameState {
    pub fn create_machine(&mut self) {
        info!("Generating all Machines");
        let sender_clone = self.sender.as_mut().unwrap().clone();
        let all = gen_all_machines();
        for m in &all {
            //code can panic @cargo bene fix
            let new_ms = Machine::new_by_const(self, sender_clone.clone(), m.clone()).unwrap();
            self.areas.push(Box::new(new_ms));
        }
    }

    pub fn draw_machines(&self, canvas: &mut Canvas, scale: Vec2, ctx: &mut Context) -> RLResult {
        for area in &self.areas {
            let machine = area.get_graphic();
            let mut pos = Vec2 {
                x: area.get_collision_area().x,
                y: area.get_collision_area().y,
            };
            draw!(canvas, &machine, pos, scale);
            let status = Mesh::new_circle(
                ctx,
                ggez::graphics::DrawMode::fill(),
                Vec2::new(0.0, 0.0),
                15.0,
                0.1,
                area.get_state().into(),
            )?;
            pos.x += 20.;
            pos.y += 20.;
            draw!(canvas, &status, pos, scale);
        }
        Ok(())
    }
}
