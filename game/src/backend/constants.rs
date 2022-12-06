//! This file contains constants that are necessary for the game.
use crate::backend::rlcolor::RLColor;
use crate::backend::utils::gen_inventory;
use crate::game_core::resources::Resources;
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
pub const HANDBOOK_TEXT: &str = "Werker macht was Sauerstoff auch Ich auch";

/// Contains the position of the time.
pub(crate) const TIME_POSITION: (f32, f32) = (1205., 960.);

#[allow(clippy::too_many_lines)]
pub(crate) fn gen_all_machines() -> [(String, Rect, Vec<Trade>, Resources<i16>); 7] {
    [
        //Die Test maschine wird zu testen des spieles genutzt. Sie gibt einem free items
        (
            "test".to_string(),
            Rect {
                x: 284.0,
                y: 230.0,
                w: 100.0,
                h: 100.0,
            },
            vec![
                Trade::new(
                    "free_items".to_string(),
                    10,
                    State::Broken,
                    State::Idle,
                    true,
                    gen_inventory(-100, -100, -100),
                ),
                Trade::new(
                    "reset_items".to_string(),
                    0,
                    State::Idle,
                    State::Running,
                    true,
                    gen_inventory(100, 97, 99),
                ),
                Trade::new(
                    "free_items".to_string(),
                    0,
                    State::Running,
                    State::Idle,
                    true,
                    gen_inventory(-100, -100, -100),
                ),
            ],
            Resources {
                oxygen: -25,
                energy: -25,
                life: -4,
            },
        ),
        //Definition Oxygen Maschine
        (
            "Oxygen".to_string(),
            Rect {
                x: 600.0,
                y: 250.0,
                w: 100.0,
                h: 100.0,
            },
            vec![
                Trade::new(
                    "repair_Oxygen".to_string(),
                    100,
                    State::Broken,
                    State::Idle,
                    false,
                    gen_inventory(2, 0, 0),
                ),
                Trade::new(
                    "start_Oxygen".to_string(),
                    0,
                    State::Idle,
                    State::Running,
                    true,
                    gen_inventory(0, 0, 0),
                ),
                Trade::new(
                    "stop_Oxygen".to_string(),
                    0,
                    State::Running,
                    State::Idle,
                    true,
                    gen_inventory(0, 0, 0),
                ),
            ],
            Resources {
                oxygen: 20,
                energy: -30,
                life: 0,
            },
        ),
        //Definition Stromgenerator Maschine
        (
            "Stromgenerator".to_string(),
            Rect {
                x: 284.0,
                y: 740.0,
                w: 200.0,
                h: 200.0,
            },
            vec![
                Trade::new(
                    "fueling_Stromgenerator".to_string(),
                    1000,
                    State::Broken,
                    State::Running,
                    true,
                    gen_inventory(0, 1, 0),
                ),
                Trade::new(
                    "start_Stromgenerator".to_string(),
                    1,
                    State::Idle,
                    State::Running,
                    true,
                    gen_inventory(0, 0, 0),
                ),
                Trade::new(
                    "stop_Stromgenerator".to_string(),
                    0,
                    State::Running,
                    State::Idle,
                    true,
                    gen_inventory(0, 0, 0),
                ),
            ],
            Resources {
                oxygen: -5,
                energy: 50,
                life: 0,
            },
        ),
        //Definition werkermaschine Maschine
        (
            "werkermaschine".to_string(),
            Rect {
                x: 600.0,
                y: 600.0,
                w: 200.0,
                h: 100.0,
            },
            vec![
                Trade::new(
                    "repair_werkermaschine".to_string(),
                    100,
                    State::Broken,
                    State::Idle,
                    false,
                    gen_inventory(0, 0, 1),
                ),
                Trade::new(
                    "produce_superglue".to_string(),
                    120,
                    State::Idle,
                    State::Running,
                    true,
                    gen_inventory(-1, 0, 0),
                ),
            ],
            Resources {
                oxygen: 0,
                energy: -15,
                life: 0,
            },
        ),
        //Definition 3d_printer Maschine
        (
            "3d_printer".to_string(),
            Rect {
                x: 1722.0,
                y: 840.0,
                w: 100.0,
                h: 100.0,
            },
            vec![
                Trade::new(
                    "repair_3d_printer".to_string(),
                    300,
                    State::Broken,
                    State::Idle,
                    false,
                    gen_inventory(2, 1, 0),
                ),
                Trade::new(
                    "produce_3d_teil".to_string(),
                    200,
                    State::Idle,
                    State::Running,
                    true,
                    gen_inventory(2, 0, -1),
                ),
            ],
            Resources {
                oxygen: 0,
                energy: -25,
                life: 0,
            },
        ),
        //Definition Kommunikationsmodul Maschine
        (
            "Kommunikationsmodul".to_string(),
            Rect {
                x: 1000.0,
                y: 230.0,
                w: 300.0,
                h: 100.0,
            },
            vec![
                Trade::new(
                    "Kommunikationsmodul_reparieren".to_string(),
                    400,
                    State::Broken,
                    State::Idle,
                    false,
                    gen_inventory(5, 0, 3),
                ),
                Trade::new(
                    "Notfall_signal_absetzen".to_string(),
                    1000,
                    State::Idle,
                    State::Running,
                    true,
                    gen_inventory(0, 0, 0),
                ),
            ],
            Resources {
                oxygen: 0,
                energy: -20,
                life: 0,
            },
        ),
        //Definition vom ersten Loch
        (
            "Loch".to_string(),
            Rect {
                x: 1722.0,
                y: 230.0,
                w: 100.0,
                h: 100.0,
            },
            vec![Trade::new(
                "repair_Loch".to_string(),
                100,
                State::Running,
                State::Idle,
                false,
                gen_inventory(2, 0, 0),
            )],
            Resources {
                oxygen: -20,
                energy: -5,
                life: -2,
            },
        ),
    ]
}
