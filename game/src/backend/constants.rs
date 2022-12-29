//! Contains all constants that are necessary for the game to played.
use crate::backend::rlcolor::RLColor;
use crate::game_core::player::gen_inventory;
use crate::game_core::resources::Resources;
use crate::languages::{machine_names, Lang};
use crate::machines::machine::{Machine, State};
use crate::machines::trade::Trade;
use ggez::graphics::{Color, Rect};

/// Contains the screen resolution of the game.
/// The game is designed to be played in 1920x1080.
/// The game will be scaled to the screen resolution of the user.
pub const SCREEN_RESOLUTION: (f32, f32) = (1920., 1080.);

/// Contains the desired FPS of the game-loop.
pub(crate) const DESIRED_FPS: u32 = 60;

/// Contains the coordinates map border( x-right, y-bottom, x-left, y-top)
pub const MAP_BORDER: [usize; 4] = [1780, 860, 270, 220];

/// Contains the position of the resource bars.
pub(crate) const RESOURCE_POSITION: [f32; 3] = [316.0, 639.0, 1373.0];

/// Contains the color used for the resource bars.
pub(crate) const COLORS: [Color; 3] = [RLColor::BLUE, RLColor::GOLD, RLColor::DARK_RED];

/// Contains the size of the player icon (in px) to scale the collision area.
pub(crate) const PLAYER_ICON_SIZE: (usize, usize) = (58, 96);

/// Contains the interaction radius of the player (in px).
pub(crate) const PLAYER_INTERACTION_RADIUS: f32 = 50.;

/// Contains the movement speed of the player (in px).
pub const MOVEMENT_SPEED: usize = 10;

/// Contains the position of the time.
pub(crate) const TIME_POSITION: (f32, f32) = (1205., 960.);

/// Change rate fot the event Sandsturm
pub(crate) const SANDSTURM_CR: Resources<i16> = Resources {
    oxygen: 10,
    energy: 0,
    life: 0,
};

#[allow(clippy::too_many_lines)]
/// Generates all machines with all their name, position, trades and resources.
/// # Returns
/// A Vector of `Machine`s
pub(crate) fn gen_all_machines(lng: Lang) -> Vec<Machine> {
    vec![
        // Oxygen machine
        Machine::new_by_const((
            machine_names(lng)[0].into(),
            machine_names(Lang::De)[0].into(),
            Rect {
                x: 280.0,
                y: 230.0,
                w: 350.0,
                h: 182.0,
            },
            vec![
                Trade::new(
                    "repair_Oxygen".into(),
                    100,
                    State::Broken,
                    State::Idle,
                    false,
                    gen_inventory(2, 0, 0, lng),
                ),
                Trade::new(
                    "start_Oxygen".into(),
                    0,
                    State::Idle,
                    State::Running,
                    true,
                    gen_inventory(0, 0, 0, lng),
                ),
                Trade::new(
                    "stop_Oxygen".into(),
                    0,
                    State::Running,
                    State::Idle,
                    true,
                    gen_inventory(0, 0, 0, lng),
                ),
            ],
            Resources {
                oxygen: 30,
                energy: -30,
                life: 0,
            },
        )),
        // Electricity machine
        Machine::new_by_const((
            machine_names(lng)[1].into(),
            machine_names(Lang::De)[1].into(),
            Rect {
                x: 282.0,
                y: 752.0,
                w: 194.0,
                h: 189.0,
            },
            vec![
                Trade::new(
                    "fueling_Stromgenerator".into(),
                    700,
                    State::Broken,
                    State::Running,
                    true,
                    gen_inventory(0, 1, 0, lng),
                ),
                Trade::new(
                    "start_Stromgenerator".into(),
                    1,
                    State::Idle,
                    State::Running,
                    true,
                    gen_inventory(0, 0, 0, lng),
                ),
                Trade::new(
                    "stop_Stromgenerator".into(),
                    0,
                    State::Running,
                    State::Idle,
                    true,
                    gen_inventory(0, 0, 0, lng),
                ),
            ],
            Resources {
                oxygen: -5,
                energy: 200,
                life: 0,
            },
        )),
        // Worker machine
        Machine::new_by_const((
            machine_names(lng)[2].into(),
            machine_names(Lang::De)[2].into(),
            Rect {
                x: 1000.0,
                y: 780.0,
                w: 300.0,
                h: 150.0,
            },
            vec![
                Trade::new(
                    "repair_werkermaschine".into(),
                    100,
                    State::Broken,
                    State::Idle,
                    false,
                    gen_inventory(0, 0, 1, lng),
                ),
                Trade::new(
                    "produce_superglue".into(),
                    120,
                    State::Idle,
                    State::Running,
                    true,
                    gen_inventory(-1, 0, 0, lng),
                ),
            ],
            Resources {
                oxygen: 0,
                energy: -15,
                life: 0,
            },
        )),
        // 3d Printer machine
        Machine::new_by_const((
            machine_names(lng)[3].into(),
            machine_names(Lang::De)[3].into(),
            Rect {
                x: 930.0,
                y: 230.0,
                w: 200.0,
                h: 148.0,
            },
            vec![
                Trade::new(
                    "repair_3d_printer".into(),
                    300,
                    State::Broken,
                    State::Idle,
                    false,
                    gen_inventory(2, 0, 0, lng),
                ),
                Trade::new(
                    "produce_3d_teil".into(),
                    200,
                    State::Idle,
                    State::Running,
                    true,
                    gen_inventory(2, 0, -1, lng),
                ),
            ],
            Resources {
                oxygen: 0,
                energy: -25,
                life: 0,
            },
        )),
        // Communication module
        Machine::new_by_const((
            machine_names(lng)[4].into(),
            machine_names(Lang::De)[4].into(),
            Rect {
                x: 1640.0,
                y: 320.0,
                w: 175.0,
                h: 477.0,
            },
            vec![
                Trade::new(
                    "Kommunikationsmodul_reparieren".into(),
                    400,
                    State::Broken,
                    State::Idle,
                    false,
                    gen_inventory(5, 0, 3, lng),
                ),
                Trade::new(
                    "Notfall_signal_absetzen".into(),
                    1000,
                    State::Idle,
                    State::Running,
                    true,
                    gen_inventory(1, 0, 1, lng),
                ),
            ],
            Resources {
                oxygen: 0,
                energy: -30,
                life: 0,
            },
        )),
        // First hole
        Machine::new_by_const((
            machine_names(lng)[5].into(),
            machine_names(Lang::De)[5].into(),
            Rect {
                x: 780.0,
                y: 230.0,
                w: 32.0,
                h: 18.0,
            },
            vec![Trade::new(
                "repair_Loch".into(),
                100,
                State::Running,
                State::Idle,
                false,
                gen_inventory(2, 0, 0, lng),
            )],
            Resources {
                oxygen: -15,
                energy: -5,
                life: 0,
            },
        )),
        // Second hole
        Machine::new_by_const((
            machine_names(lng)[6].into(),
            machine_names(Lang::De)[6].into(),
            Rect {
                x: 680.0,
                y: 900.0,
                w: 32.0,
                h: 18.0,
            },
            vec![Trade::new(
                "repair_Loch".into(),
                100,
                State::Running,
                State::Idle,
                false,
                gen_inventory(2, 0, 0, lng),
            )],
            Resources {
                oxygen: -15,
                energy: -5,
                life: 0,
            },
        )),
    ]
}
