use std::fmt::Debug;
use crate::error;
type RedResult<T = ()> = Result<T, error::RedError>;
/// A screen is every drawable object in the game, so the main menu is a screen too
pub trait Screen: Debug {
    fn update(&mut self) -> RedResult;
    fn draw(&self) -> RedResult;
}

/// Now we can use this trait to make for example a scene
pub struct Screenstack {
    screens: Vec<Box<dyn Screen>>,
}