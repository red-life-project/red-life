//!DIESE DATEI IST ZUM TESTEN VON SANDER
use crate::backend::gamestate::GameState;

use crate::machines::machine::Machine;

use crate::backend::constants::gen_all_machines;
use crate::backend::rlcolor::RLColor;
use crate::{draw, RLResult};
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Mesh, Rect};
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
            self.machines.push(new_ms);
        }
    }

    pub fn draw_machines(&self, canvas: &mut Canvas, scale: Vec2, ctx: &mut Context) -> RLResult {
        for machine in &self.machines {
            let image = machine.get_graphic();
            let mut pos = Vec2 {
                x: machine.get_collision_area().x,
                y: machine.get_collision_area().y,
            };
            draw!(canvas, &image, pos, scale);
            // Draws the machine status on top of the machine
            let status = Mesh::new_circle(
                ctx,
                ggez::graphics::DrawMode::fill(),
                Vec2::new(0.0, 0.0),
                15.0,
                0.1,
                machine.get_state().into(),
            )?;
            // Draws the machine timer on top of the machine
            let time = machine.get_time_percentage();
            
                pos.x += 20.;
                pos.y += 20.;
                draw!(canvas, &status, pos, scale);
            if time > 0. {
                // Bar for machine Timer
                pos.x += 40.;
                pos.y -= 30.;
                let rect1 = Mesh::new_rounded_rectangle(
                    ctx,
                    ggez::graphics::DrawMode::fill(),
                    Rect::new(0.0, 0.0, 100.0, 10.0),
                    15.,
                    RLColor::DARK_GREY,
                )?;
                draw!(canvas, &rect1, pos, scale);
                // Bar of current time
                let rect2 = Mesh::new_rounded_rectangle(
                    ctx,
                    ggez::graphics::DrawMode::fill(),
                    Rect::new(0.0, 0.0, 100.0 * time, 10.0),
                    15.,
                    RLColor::BLACK,
                )?;
                draw!(canvas, &rect2, pos, scale);
            }
        }
        Ok(())
    }
}
