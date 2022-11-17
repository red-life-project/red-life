use crate::game_core::player::Player;
use dyn_clone::DynClone;
use ggez::graphics::Rect;
use std::fmt::Debug;

pub trait Area: DynClone + Debug {
    fn interact(&mut self, player: &Player);
    fn get_collision_area(&self) -> Rect;
    fn get_interaction_area(&self) -> Rect;
}
