//! This File handels code surrounding Machine with in `GameState`
use crate::backend::constants::gen_all_machines;
use crate::backend::gamestate::GameState;
use crate::backend::rlcolor::RLColor;
use crate::backend::utils::get_draw_params;
use crate::{draw, RLResult};
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Mesh, Rect};
use ggez::Context;
use tracing::info;

impl GameState {
    /// Creates all Machines for initial creation and pushes them into a list
    pub fn create_machine(&mut self) {
        info!("Generating all Machines");
        self.machines = gen_all_machines();
    }

    /// Paints the machine sprites and if applicable it shows the state or time remaining
    /// # Arguments
    /// * `canvas`: The canvas to draw on
    /// * `scale`: The scale of the canvas
    /// * `ctx`: The `Context` of the game
    /// # Returns
    /// * `RLResult`: A `RLResult` to validate the success of the paint function
    pub fn draw_machines(&self, canvas: &mut Canvas, scale: Vec2, ctx: &mut Context) -> RLResult {
        for machine in &self.machines {
            let image = machine.get_graphic();
            let mut pos = Vec2 {
                x: machine.hitbox.x,
                y: machine.hitbox.y,
            };
            draw!(canvas, image, pos, scale);
            if !machine.name.contains("Loch") {
                // Draws the machine status on top of the machine
                let status = Mesh::new_circle(
                    ctx,
                    ggez::graphics::DrawMode::fill(),
                    Vec2::new(0., 0.),
                    15.0,
                    0.1,
                    machine.state.clone().into(),
                )?;
                pos.x += 20.;
                pos.y += 20.;
                draw!(canvas, &status, pos, scale);
            };
            // Draws the machine timer on top of the machine
            let time = machine.get_time_percentage();
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
