//! Contains constants for the language "English".

/// Constant for item `Gedrucktesteil`.
pub const GEDRUCKTESTEIL: [&str; 3] = [
    "3D-printed part",
    "A 3D-printed part that can be used to repair the communication module",
    "3D-gedrucktes-Teil.png",
];
/// Constant for the item `Superglue`
pub const SUPER_GLUE: [&str; 3] = [
    "SuperGlue",
    "SuperGlue can be used to repair the machines or holes",
    "SuperGlue.png",
];
/// Constant for the item `Benzin`
pub const PETROL: [&str; 3] = [
    "Petrol",
    "Petrol can be used with the emergency generator to generate electricity",
    "Benzin.png",
];

/// Constant for the resource names.
pub(crate) const RESOURCE_NAME: [&str; 3] = ["Air", "Energy", "Life"];

/// The text for the warning-`Popup`s that appears in the top left corner.
pub const WARNINGS: [&str; 4] = [
    "A comet is on its way!",
    "The power's out!",
    "A sandstorm is on its way!",
    "A machine is down!",
];
/// The text for the mars-info-`Popup`s that appears in the top left corner.
pub const MARS_INFO: [&str; 5] = [
    "Mars is the 4th planet in our solar system",
    "Mars is one of the Earth-like planets",
    "The diameter of Mars is just under 6800 km",
    "The mass of Mars is about one-tenth that of Earth",
    "The distance to Mars is on average 228 million km",
];
/// The text for the nasa-info-`Popup`s that appears in the top left corner.
pub const NASA_INFO: [&str; 5] = [
    "NASA stands for: National Aeronautics and Space Administration",
    "NASA was founded in 1958",
    "NASA is headquartered in Washington, D.C.",
    "As part of the Apollo missions, NASA succeeded in putting the first man on the moon",
    " NASA has over 17,000 employees",
];
/// The text for the game-info-`Popup`s that appears in the top left corner.
pub const GAME_INFO: [&'static str; 1] = ["Life regeneration started"];

/// Constants for all strings used in deathscreen
pub const AIR_STRING: &str = "too little air";
pub const ENERGY_STRING: &str = "Cold";
pub const AIR_AND_ENERGY_STRING: &str = "Cold and too little air";
pub const DEATH_REASON_STRING: &str = "You died of";
pub const ADDITIONAL_INFO_STRING: &str = "Please press ESC!";
pub const RESUME_ERROR_STRING: &str = "You need a score first";

/// Constant for all strings used in `IntroScreen`
pub const INTRO_TEXT: &str =
    "You are stranded on Mars and must survive.\nTo do that, you must restore \
Hopefully, you will be able to repair the communications so you can be rescued.";

pub const TUTORIAL_TEXT: &str =
    "Move around with WASD. Interact with E.\nFor reference, you have your manual on H.";

/// Constant for the Text used in the `Button` info
pub const BUTTON_INFO: &str = "Please press the space bar!";

/// Constants for all strings used in `WinningScreen`
pub const WINNING_TEXT: &str = "You've been saved!";

/// Constants for the events that can occur.
pub const COMETA_STRIKE: [&str; 2] = [
    "COMETA STRIKE",
    "A cometa strike has hit the earth and created a hole in the wall",
];
pub const SANDSTORM: [&str; 2] = [
    "Sandsturm",
    "A sandstorm, which leads to a malfunction of the oxygen generator",
];
pub const INFORMATIONS_POPUP_NASA: [&str; 2] = [
    "InformationspopupNASA",
    "An information pop-up about NASA containing facts and information about NASA",
];
pub const POWER_FAILURE: [&str; 2] = [
    "Power failure",
    "A power failure resulting in a malfunction of the oxygen generator",
];
pub const INFORMATIONS_POPUP_MARS: [&str; 2] = [
    "InformationspopupMars",
    "An information popup about Mars containing facts and information about Mars",
];
/// Constants for the trade conflict.
pub const TRADE_CONFLICT_POPUP: [&str; 1] =
    ["The following items are missing to execute the trade:"];
/// Constants for the `time_name`.
pub const TIME_NAME: [&str; 1] = ["Time"];
/// Constants for the text of the button in the main menu
pub const BUTTON_TEXT: [&str; 4] = ["Continue", "New Game", "Exit", "German"];
/// Contains all machine names as a vec of strings.
pub(crate) const MACHINE_NAMES: [&str; 7] = [
    "Oxygen generator",
    "power generator",
    "work machine",
    "3D printer",
    "communication module",
    "hole",
    "Hole",
];
/// Contains the Messages that are displayed in the Handbook
pub(crate) const FIRST_MILESTONE_HANDBOOK_TEXT: [&str; 10] = [
    "- Repair the oxygen generator (top left)",
    "- Repair the electricity generator (bottom left)",
    "- Comets create holes in the walls",
    "- Repair holes with SuperGlue",
    "- In case of a power failure",
    "you must restart the power generator",
    "- Remember to use petrol sparingly",
    "- You can stop the generator briefly,",
    " if you have enough power",
    "\n\n Press H to close",
];

pub(crate) const SECOND_MILESTONE_HANDBOOK_TEXT: [&str; 7] = [
    "- Repair the communication system (right)",
    "- Send a message to be rescued",
    "- Your power may still fail,",
    "while you're sending the message!",
    "- When you send the message,",
    "you automatically win.",
    "\n\n Press H to close",
];
pub(crate) const SEND_MSG_FAILURE: &str =
    "The message cannot be sent until the system is restored.";
