//! Contains all constants that are necessary for the game to played.
use crate::backend::rlcolor::RLColor;
use crate::game_core::player::gen_inventory;
use crate::game_core::resources::Resources;
use crate::languages::{machine_names, Lang, power_failure, warnings, nasa_info, mars_info};
use crate::machines::machine::{Machine, State};
use crate::machines::trade::Trade;
use ggez::graphics::{Color, Rect};

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum PopupType {
    Warning,
    Nasa,
    Mars,
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ObjectId {
    OxygenGenerator = 0,
    PowerGenerator = 1,
    WorkMachine = 2,
    Printer3D = 3,
    CommunicationModule = 4,
    NorthHole = 5,
    SouthHole = 6,
}

impl ObjectId {
    pub fn t(self, lng: Lang) -> &'static str {
        machine_names(lng)[self as usize]
    }

    pub fn is_hole(self) -> bool {
        matches!(self, ObjectId::NorthHole | ObjectId::SouthHole)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TradeId {
    NoTrade ,
    RepairOxygen,
    StartOxygen,
    StopOxygen,
    FuelingPowerGenerator,
    StartPowerGenerator,
    StopPowerGenerator,
    RepairWorkMachine,
    ProduceSuperglue,
    Repair3dPrinter,
    Produce3dPart,
    RepairCommunicationModule ,
    EmergencySignalOff ,
    RepairNorthHole ,
    RepairSouthHole ,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum EventId {
    Sandstorm,
    CometStrike,
    PowerFailure,
    InformationPopupMars,
    InformationPopupNasa,
}

impl EventId {
    pub fn warning(self, lng: Lang, message_id: usize) -> &'static str {
        match self {
            EventId::Sandstorm => warnings(lng)[2],
            EventId::CometStrike => warnings(lng)[0],
            EventId::PowerFailure => power_failure(lng)[1],
            EventId::InformationPopupMars => nasa_info(lng)[message_id],
            EventId::InformationPopupNasa => mars_info(lng)[message_id],
        }
    }
}

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
pub(crate) const SANDSTORM_CR: Resources<i16> = Resources {
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
            ObjectId::OxygenGenerator,
            Rect {
                x: 280.0,
                y: 230.0,
                w: 350.0,
                h: 182.0,
            },
            vec![
                Trade::new(
                    TradeId::RepairOxygen,
                    100,
                    State::Broken,
                    State::Idle,
                    false,
                    gen_inventory(2, 0, 0, lng),
                ),
                Trade::new(
                    TradeId::StartOxygen,
                    0,
                    State::Idle,
                    State::Running,
                    true,
                    gen_inventory(0, 0, 0, lng),
                ),
                Trade::new(
                    TradeId::StopOxygen,
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
            ObjectId::PowerGenerator,
            Rect {
                x: 282.0,
                y: 752.0,
                w: 194.0,
                h: 189.0,
            },
            vec![
                Trade::new(
                    TradeId::FuelingPowerGenerator,
                    700,
                    State::Broken,
                    State::Running,
                    true,
                    gen_inventory(0, 1, 0, lng),
                ),
                Trade::new(
                    TradeId::StartPowerGenerator,
                    1,
                    State::Idle,
                    State::Running,
                    true,
                    gen_inventory(0, 0, 0, lng),
                ),
                Trade::new(
                    TradeId::StopPowerGenerator,
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
            ObjectId::WorkMachine,
            Rect {
                x: 1000.0,
                y: 780.0,
                w: 300.0,
                h: 150.0,
            },
            vec![
                Trade::new(
                    TradeId::RepairWorkMachine,
                    100,
                    State::Broken,
                    State::Idle,
                    false,
                    gen_inventory(0, 0, 1, lng),
                ),
                Trade::new(
                    TradeId::ProduceSuperglue,
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
            ObjectId::Printer3D,
            Rect {
                x: 930.0,
                y: 230.0,
                w: 200.0,
                h: 148.0,
            },
            vec![
                Trade::new(
                    TradeId::Repair3dPrinter,
                    300,
                    State::Broken,
                    State::Idle,
                    false,
                    gen_inventory(2, 0, 0, lng),
                ),
                Trade::new(
                    TradeId::Produce3dPart,
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
            ObjectId::CommunicationModule,
            Rect {
                x: 1640.0,
                y: 320.0,
                w: 175.0,
                h: 477.0,
            },
            vec![
                Trade::new(
                    TradeId::RepairCommunicationModule,
                    400,
                    State::Broken,
                    State::Idle,
                    false,
                    gen_inventory(5, 0, 3, lng),
                ),
                Trade::new(
                    TradeId::EmergencySignalOff,
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
            ObjectId::NorthHole,
            Rect {
                x: 780.0,
                y: 230.0,
                w: 32.0,
                h: 18.0,
            },
            vec![Trade::new(
                TradeId::RepairNorthHole,
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
            ObjectId::SouthHole,
            Rect {
                x: 680.0,
                y: 900.0,
                w: 32.0,
                h: 18.0,
            },
            vec![Trade::new(
                TradeId::RepairSouthHole,
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
