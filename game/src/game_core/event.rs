use crate::backend::popup_messages::{MARS_INFO, NASA_INFO, WARNINGS};
use crate::backend::screen::{Popup, StackCommand};
use ggez::graphics::Color;
use serde::{Deserialize, Serialize};
use std::sync::mpsc::Sender;
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
    pub fn new(
        event: [&str; 2],
        sender: &Sender<StackCommand>,
        popup_message: &str,
        popup_type: &str,
    ) -> Self {
        Self::send_popup(popup_message, sender, popup_type, event[0]);
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
    pub fn event_generator(popup_sender: &Sender<StackCommand>) -> Option<Event> {
        let rng = fastrand::Rng::new();
        let event = rng.usize(..50);
        match event {
            0 => Some(Event::new(
                KOMETENEINSCHLAG,
                popup_sender,
                WARNINGS[0],
                "warning",
            )),
            11 => Some(Event::new(
                INFORMATIONSPOPUP_NASA,
                popup_sender,
                NASA_INFO[rng.usize(..4)],
                "nasa",
            )),
            22 => Some(Event::new(SANDSTURM, popup_sender, WARNINGS[2], "warning")),
            33 => Some(Event::new(
                STROMAUSFALL,
                popup_sender,
                WARNINGS[1],
                "warning",
            )),
            44 => Some(Event::new(
                INFORMATIONSPOPUP_MARS,
                popup_sender,
                MARS_INFO[rng.usize(..5)],
                "mars",
            )),
            _ => None,
        }
    }
    pub fn restore_event() -> Option<Event> {
        info!("Event X restored"); // Fill missing parameters
                                   //Sender is missing -> Should send reverse of event cr
        None
    }
    /// Sends a popup of an event to the screen
    pub fn send_popup(
        popup_message: &str,
        sender: &Sender<StackCommand>,
        popup_type: &str,
        event_name: &str,
    ) {
        let popup = match popup_type {
            "warning" => Popup::warning(popup_message.to_string()),
            "nasa" => Popup::nasa(popup_message.to_string()),
            "mars" => Popup::mars(popup_message.to_string()),
            _ => Popup::new(Color::RED, "Error".to_string(), 10),
        };
        sender.send(StackCommand::Popup(popup)).unwrap();
        info!(
            "Event Popup sent: name: {}, Popup-Message: {}, Popup-Type: {}",
            event_name,
            popup_message.to_string(),
            popup_type
        );
    }
}
