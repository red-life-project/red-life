use crate::backend::area::Area;
use crate::backend::gamestate::GameState;
use crate::machines::machine::Mashine;
use crate::{draw, RLResult};
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, DrawMode};
use ggez::{graphics, Context};

///DIESE DATEI IST ZUM TESTEN VON SANDER

impl GameState {
    pub fn create_machine(&mut self) {
        dbg!(format!("create_machine",));

        let new_ms = Mashine::default(self);
        self.machines.push(new_ms.clone()); // TODO:RM
        self.areas.push(Box::new(new_ms));
    }

    pub fn draw_machines(&self, canvas: &mut Canvas, scale: Vec2, ctx: &mut Context) -> RLResult {
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
            let machine = area.get_graphic();
            let pos = Vec2 {
                x: area.get_collision_area().x,
                y: area.get_collision_area().y,
            };
            draw!(canvas, &machine, pos, scale);
        }
        Ok(())
    }
}
