use crate::backend::utils::is_colliding;
use crate::game_core::player::Player;
use dyn_clone::DynClone;
use ggez::glam::Vec2;
use ggez::graphics::{Image, Rect};
use std::fmt::Debug;

pub trait Area: DynClone + Debug {
    fn interact(&mut self, player: &Player);
    fn is_interactable(&self, pos: (usize, usize)) -> bool {
        is_colliding(pos, &self.get_interaction_area())
    }
    fn get_collision_area(&self) -> Rect;
    fn get_interaction_area(&self) -> Rect;
    fn get_graphic(&self)->&Image;
}
