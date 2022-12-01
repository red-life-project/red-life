//!DIESE DATEI IST ZUM TESTEN VON SANDER
use crate::backend::gamestate::GameState;

use crate::game_core::item::Item;
use crate::game_core::resources::Resources;
use crate::languages::german::{BENZIN, GEDRUCKTESTEIL};
use crate::machines::machine::{Machine, State};
use crate::machines::trade::Trade;
use crate::{draw, RLResult};
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Mesh, Rect};
use ggez::Context;
use tracing::info;

impl GameState {
    pub fn create_machine(&mut self) -> RLResult {
        info!("Generating all Machines");
        let sender_clone = self.sender.as_mut().unwrap().clone();
        let new_ms = Machine::quick(self, sender_clone.clone())?;
        self.areas.push(Box::new(new_ms));

        let clone = self.player.inventory.clone();
        let ms_2 = Machine::new(
            self,
            "Oxygen".to_string(),
            Rect {
                x: 600.0,
                y: 300.0,
                w: 100.0,
                h: 100.0,
            },
            vec![
                Trade::new_and_set(
                    "repair_test".to_string(),
                    100,
                    State::Broken,
                    State::Idle,
                    &mut clone.clone(),
                    (2, 2, 2),
                    Item::new(BENZIN),
                    0,
                ),
                Trade::new_and_set(
                    "repair_test".to_string(),
                    100,
                    State::Idle,
                    State::Running,
                    &mut clone.clone(),
                    (0, 1, 2),
                    Item::new(BENZIN),
                    0,
                ),
                Trade::new_and_set(
                    "repair_test".to_string(),
                    100,
                    State::Running,
                    State::Idle,
                    &mut clone.clone(),
                    (0, 0, 0),
                    Item::new(GEDRUCKTESTEIL),
                    1,
                ),
            ],
            Resources {
                oxygen: 10,
                energy: -5,
                life: 0,
            },
            sender_clone,
        )?;

        self.areas.push(Box::new(ms_2));

        Ok(())
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
