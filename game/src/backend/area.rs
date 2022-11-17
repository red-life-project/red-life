use std::fmt::Debug;
use dyn_clone::DynClone;
use ggez::graphics::Rect;
use crate::backend::gamestate::Player;

pub trait Area: DynClone + Debug{
    fn interact(&mut self);
    fn get_collision_area(&self) -> Rect;
    fn get_interaction_area(&self) -> Rect;
}