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
pub const AIR_STRING: &str = "zu wenig Luft";
pub const ENERGY_STRING: &str = "Kälte";
pub const AIR_AND_ENERGY_STRING: &str = "Kälte und zu wenig Luft";
pub const DEATH_REASON_STRING: &str = "Du bist gestorben an";
pub const ADDITIONAL_INFO_STRING: &str = "Bitte drücke ESC!";
pub const RESUME_ERROR_STRING: &str = "Du brauchst zuerst einen Spielstand";

/// Constant for all strings used in `IntroScreen`
pub const INTRO_TEXT: &str = "Du bist auf dem Mars gestrandet und musst überleben.\nDazu musst du die \
Sauerstoffproduktion wiederherstellen.\nHoffentlich schaffst du es, die Kommunikation zu reparieren, \ndamit du gerettet werden kannst.\n";
pub const TUTORIAL_TEXT: &str =
    "Bewege dich mit WASD. Interagiere mit E.\nZum Nachschlagen hast du auf H dein Handbuch.";

/// Constant for the Text used in the `Button` info
pub const BUTTON_INFO: &str = "Bitte drücke die Leertaste!";

/// Constants for all strings used in `WinningScreen`
pub const WINNING_TEXT: &str = "Du wurdest gerettet!";

/// Constants for the events that can occur.
pub const KOMETENEINSCHLAG: [&str; 2] = [
    "KOMETENEINSCHLAG",
    "Ein KOMETENEINSCHLAG hat die Erde getroffen und hat ein Loch in der Wand erzeugt",
];
pub const SANDSTURM: [&str; 2] = [
    "Sandsturm",
    "Ein Sandsturm, welcher zu einer Störung des Sauerstoffgenerators führt",
];
pub const INFORMATIONSPOPUP_NASA: [&str; 2] = [
    "InformationspopupNASA",
    "Ein Informationspopup über die NASA, welches Fakten und Informationen über die NASA enthält",
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
/// Constants for the `time_name`.
pub const TIME_NAME: [&str; 1] = ["Zeit"];
/// Constants for the text of the button in the main menu
pub const BUTTON_TEXT: [&str; 3] = ["Fortsetzen", "Neues Spiel", "Beenden"];
/// Contains all machine names as a vec of strings.
pub(crate) const MACHINE_NAMES: [&str; 7] = [
    "Sauerstoffgenerator",
    "Stromgenerator",
    "Werkermaschine",
    "3D-Drucker",
    "Kommunikationsmodul",
    "Loch",
    "Loch",
];
/// Contains the Messages that are displayed in the Handbook
pub(crate) const FIRST_MILESTONE_HANDBOOK_TEXT: [&str; 10] = [
    "- Repariere den Sauerstoffgenerator (oben links)",
    "- Repariere den Stromgenerator (unten links)",
    "- Kometen erzeugen Löcher in den Wänden",
    "- Löcher werden mit SuperGlue repariert",
    "- Bei einem Stromausfall",
    "   musst du den Stromgenerator erneut starten.",
    "- Denk daran sparsam mit Benzin umzugehen!",
    "- Du kannst den Generator kurz anhalten,",
    "   wenn du genug Energie hast",
    "\n\n           Drücke H zum schließen",
];

pub(crate) const SECOND_MILESTONE_HANDBOOK_TEXT: [&str; 7] = [
    "- Repariere das Kommunikationssystem (rechts)",
    "- Sende eine Nachricht ab um gerettet zu werden",
    "- Dein Strom kann immernoch ausfallen,",
    "   während du die Nachricht sendest!",
    "- Wenn du die Nachricht abgeschickt hast,",
    "   gewinnst du automatisch.",
    "\n\n             Drücke H zum schließen",
];
