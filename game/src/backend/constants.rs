//! This file contains constants that are necessary for the game.
use crate::backend::rlcolor::RLColor;
use ggez::graphics::Color;

/// Contains the screen resolution of the game.
pub const SCREEN_RESOLUTION: (f32, f32) = (1920., 1080.);

/// Contains the desired FPS of the game-loop.
pub(crate) const DESIRED_FPS: u32 = 60;

/// Contains the map border( x-right, y-bottom, x-left, y-top)
pub const MAP_BORDER: [usize; 4] = [1750, 850, 255, 220];

/// Contains the position of the resource bars.
pub(crate) const RESOURCE_POSITION: [f32; 3] = [316.0, 639.0, 1373.0];

/// Contains the color used for the resource bars.
pub(crate) const COLORS: [Color; 3] = [RLColor::BLUE, RLColor::GOLD, RLColor::DARK_RED];

/// Contains the size of the player icon to scale the collision area.
pub(crate) const PLAYER_ICON_SIZE: (usize, usize) = (60, 50);

// pub const MACHINE_POSITIONS: [[i32; 4]; 4] = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
