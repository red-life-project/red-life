use crate::backend::utils::is_colliding;
use crate::game_core::item::Item;
use crate::game_core::player::Player;
use ggez::graphics::{Image, Rect};
use std::fmt::Debug;

pub trait Area: Debug {
    fn interact(&mut self, player :  &mut Player) -> Player;
    fn is_interactable(&self, pos: (usize, usize)) -> bool {
        is_colliding(pos, &self.get_interaction_area())
    }
    fn get_collision_area(&self) -> Rect;
    fn get_interaction_area(&self) -> Rect;
    fn get_graphic(&self) -> Image;
    fn is_non_broken_machine(&self) -> bool;
    fn get_name(&self) -> String;
}
