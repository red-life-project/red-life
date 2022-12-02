//! This file contains constants that are necessary for the game.
use crate::backend::rlcolor::RLColor;
use crate::backend::utils::gen_inventory;
use crate::game_core::item::Item;
use crate::game_core::resources::Resources;
use crate::languages::german::{BENZIN, GEDRUCKTESTEIL};
use crate::machines::machine::State;
use crate::machines::trade::Trade;
use ggez::graphics::{Color, Rect};
use std::string::ToString;

/// Contains the screen resolution of the game.
pub const SCREEN_RESOLUTION: (f32, f32) = (1920., 1080.);

/// Contains the desired FPS of the game-loop.
pub(crate) const DESIRED_FPS: u32 = 60;

/// Contains the map border( x-right, y-bottom, x-left, y-top)
pub const MAP_BORDER: [usize; 4] = [1780, 860, 270, 220];

/// Contains the position of the resource bars.
pub(crate) const RESOURCE_POSITION: [f32; 3] = [316.0, 639.0, 1373.0];

/// Contains the color used for the resource bars.
pub(crate) const COLORS: [Color; 3] = [RLColor::BLUE, RLColor::GOLD, RLColor::DARK_RED];

/// Contains the size of the player icon to scale the collision area.
pub(crate) const PLAYER_ICON_SIZE: (usize, usize) = (58, 96);

/// Contains the interaction radius of the player.
pub(crate) const PLAYER_INTERACTION_RADIUS: f32 = 50.;
// pub const MACHINE_POSITIONS: [[i32; 4]; 4] = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];

//"its a const lol" // problem ist das ich nicht vec![] in const aufrufen darf und es wäre anstrengend
pub(crate) fn gen_all_machines() -> [(String, Rect, Vec<Trade>, Resources<i16>); 2] {
    [
        //("BEISPIEL".to_string() , Rect::default(), vec![Trade::default()], Resources::default()),
        (
            "test".to_string(),
            Rect {
                x: 300.0,
                y: 200.0,
                w: 100.0,
                h: 100.0,
            },
            vec![
                Trade::new(
                    "repair_test".to_string(),
                    0,
                    State::Broken,
                    State::Idle,
                    gen_inventory(-1, -1, -1),
                    Item::new(BENZIN),
                    0,
                ),
                Trade::new(
                    "repair_test".to_string(),
                    100,
                    State::Idle,
                    State::Running,
                    gen_inventory(1, 1, 1),
                    Item::new(BENZIN),
                    0,
                ),
                Trade::new(
                    "repair_test".to_string(),
                    0,
                    State::Running,
                    State::Idle,
                    gen_inventory(-2, -2, -2),
                    Item::new(GEDRUCKTESTEIL),
                    1,
                ),
            ],
            Resources {
                oxygen: 0,
                energy: -25,
                life: 0,
            },
        ),
        (
            "Oxygen".to_string(),
            Rect {
                x: 600.0,
                y: 200.0,
                w: 100.0,
                h: 100.0,
            },
            vec![
                Trade::new(
                    "repair_test".to_string(),
                    0,
                    State::Broken,
                    State::Idle,
                    gen_inventory(2, 2, 2),
                    Item::new(BENZIN),
                    0,
                ),
                Trade::new(
                    "repair_test".to_string(),
                    0,
                    State::Idle,
                    State::Running,
                    gen_inventory(0, 1, 2),
                    Item::new(BENZIN),
                    0,
                ),
                Trade::new(
                    "repair_test".to_string(),
                    100,
                    State::Running,
                    State::Idle,
                    gen_inventory(0, 0, 0),
                    Item::new(GEDRUCKTESTEIL),
                    1,
                ),
            ],
            Resources {
                oxygen: 0,
                energy: -25,
                life: 0,
            },
        ),
    ]
}
