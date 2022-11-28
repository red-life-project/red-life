use crate::backend::rlcolor::RLColor;
use ggez::graphics::Color;

/// This file contains constants that are necessary for the game.

/// Contains the map border( x-right, y-bottom, x-left, y-top)
pub const MAP_BORDER: [usize; 4] = [1750, 850, 255, 220];

/// Contains the position of the resource bars.
pub(crate) const RESOURCE_POSITION: [f32; 3] = [316.0, 639.0, 1373.0];

/// Contains the color used for the resource bars.
pub(crate) const COLORS: [Color; 3] = [RLColor::BLUE, RLColor::GOLD, RLColor::DARK_RED];
// pub const MACHINE_POSITIONS: [[i32; 4]; 4] = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
