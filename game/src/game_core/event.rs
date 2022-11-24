use serde::{Deserialize, Serialize};
use tracing::info;

pub const KOMETENEINSCHLAG: [&str; 2] = [
    "KOMETENEINSCHLAG",
    "Ein KOMETENEINSCHLAG hat die Erde getroffen und hat ein Loch in der Wand erzeugt",
];
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
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct Event {
    name: String,
    info_text: String,
}
impl Event {
    pub fn new(event: [&str; 2]) -> Self {
        info!(
            "New event created: {}, info text: {}",
            event[0].to_string(),
            event[1].to_string()
        );
        Self {
            name: event[0].to_string(),
            info_text: event[1].to_string(),
        }
    }

    /// if no Event is active it either chooses a random event of the Event enum or nothing every 60 seconds
    pub fn event_generator() -> Option<Event> {
        let rng = fastrand::Rng::new();
        let event = rng.usize(..50);
        match event {
            0 => Some(Event::new(KOMETENEINSCHLAG)),
            11 => Some(Event::new(INFORMATIONSPOPUP_NASA)),
            22 => Some(Event::new(SANDSTURM)),
            33 => Some(Event::new(STROMAUSFALL)),
            44 => Some(Event::new(INFORMATIONSPOPUP_MARS)),
            _ => None,
        }
    }
    pub fn restore_event() -> Option<Event> {
        info!("Event X restored"); // Fill missing parameters
                                   //Sender is missing -> Should send reverse of event cr
        None
    }
}
