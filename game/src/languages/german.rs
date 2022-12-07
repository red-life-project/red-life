//! Contains constants for the language "German".

/// Constant for item `Gedrucktesteil`.
pub const GEDRUCKTESTEIL: [&str; 3] = [
    "3D-gedrucktes-Teil",
    "Ein 3D-gedrucktes-Teil, welches zur Reparatur des Kommunikationsmoduls verwendet werden kann",
    "3D-gedrucktes-Teil.png",
];
/// Constant for the item `Superglue`
pub const SUPER_GLUE: [&str; 3] = [
    "SuperGlue",
    "SuperGlue kann zur Reparatur der Maschinen oder Löcher verwendet werden",
    "SuperGlue.png",
];
/// Constant for the item `Benzin`
pub const BENZIN: [&str; 3] = [
    "Benzin",
    "Benzin kann mit dem Notstromgenerator verwendet werden um Strom zu generieren",
    "Benzin.png",
];

/// Constant for the resource names.
pub(crate) const RESOURCE_NAME: [&str; 3] = ["Luft", "Energie", "Leben"];

/// The text for the warning-`Popup`s that appears in the top left corner.
pub const WARNINGS: [&str; 4] = [
    "Ein Komet ist auf dem Weg!",
    "Der Strom ist ausgefallen!",
    "Ein Sandsturm ist auf dem Weg!",
    "Eine Maschine ist ausgefallen!",
];
/// The text for the mars-info-`Popup`s that appears in the top left corner.
pub const MARS_INFO: [&str; 5] = [
    "Der Mars ist der 4. Planet in unserem Sonnensystem",
    "Der Mars zählt zu den erdähnlichen Planeten",
    "Der Durchmesser des Mars beträgt knapp 6800 km",
    "Die Masse des Mars beträgt etwa ein Zehntel der Erdmasse",
    "Die Entfernung zum Mars beträgt durchschnittlich 228 Millionen km",
];
/// The text for the nasa-info-`Popup`s that appears in the top left corner.
pub const NASA_INFO: [&str; 5] = [
    "NASA steht für: National Aeronautics and Space Administration",
    "Die NASA wurde 1958 gegründet",
    "Die NASA hat ihren Sitz in Washington D.C.",
    "Im Rahmen der Apollo-Missionen gelang es der NASA, den ersten Menschen auf den Mond zu bringen"," Die NASA hat über 17.000 Mitarbeiter"
];
/// The text for the game-info-`Popup`s that appears in the top left corner.
pub const GAME_INFO: [&str; 1] = ["Lebensregeneration gestartet"];

/// Constants for all strings used in deathscreen
pub const AIR_STRING: &str = "Luft";
pub const ENERGY_STRING: &str = "Energie";
pub const AIR_AND_ENERGY_STRING: &str = "Luft und Energie";
pub const DEATH_REASON_STRING: &str = "Dein Todesgrund: ";
pub const ADDITIONAL_INFO_STRING: &str = "Bitte drücke ESC!";

/// Constant for all strings used in IntroScreen
pub const INTRO_TEXT: &str = "Du bist auf dem Mars gestrandet und musst überleben.\nDazu musst du die \
Sauerstoffproduktion wiederherstellen.\nHoffentlich schaffst du es die Kommunikation zu reparieren, \ndamit du gerettet werden kannst.\n";
pub const TUTORIAL_TEXT: &str =
    "Bewege dich mit WASD. Interagiere mit E.\nZum Nachschlagen hast du auf H dein Handbuch.";

/// Constant for the Text used in the Button info
pub const BUTTON_INFO: &str = "Bitte drücke die Leertaste!";

/// Constants for all strings used in WinningScreen
pub const WINNING_TEXT: &str = "Du wurdest gerettet!";

/// Constants for the events that can occur.
pub const KOMETENEINSCHLAG: [&str; 2] = [
    "KOMETENEINSCHLAG",
    "Ein KOMETENEINSCHLAG hat die Erde getroffen und hat ein Loch in der Wand erzeugt",
];
/// Constants for the events that can occur.
pub const INFORMATIONSPOPUP_NASA: [&str; 2] = [
    "InformationspopupNASA",
    "Ein Informationspopup über die NASA, welches Fakten und Informationen über die NASA enthält",
];
pub const SANDSTURM: [&str; 2] = [
    "Sandsturm",
    "Ein Sandsturm, welcher zu einer Störung des Sauerstoffgenerators führt",
];
pub const STROMAUSFALL: [&str; 2] = [
    "Stromausfall",
    "Ein Stromausfall, welcher zu einer Störung des Sauerstoffgenerators führt",
];
pub const INFORMATIONSPOPUP_MARS: [&str; 2] = [
    "InformationspopupMars",
    "Ein Informationspopup über Mars, welches Fakten und Informationen über den Mars enthält",
];
/// Constants for the trade conflict.
pub const TRADE_CONFLICT_POPUP: [&str; 1] = ["Es fehlen folgende Items, um den Trade auszuführen:"];
/// Constants for the time_name.
pub const TIME_NAME: [&str; 1] = ["Zeit"];
/// Contains all machine names as a vec of strings.
pub(crate) const MACHINE_NAMES: [&str; 7] = [
    "test",
    "Sauerstoffgenerator",
    "Stromgenerator",
    "Werkermaschine",
    "3D-Drucker",
    "Kommunikationsmodul",
    "Loch",
];
