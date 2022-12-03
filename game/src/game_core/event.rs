use crate::backend::gamestate::GameState;
use crate::backend::screen::{Popup, StackCommand};
use crate::game_core::resources::Resources;
use crate::languages::german::{
    INFORMATIONSPOPUP_MARS, INFORMATIONSPOPUP_NASA, KOMETENEINSCHLAG, SANDSTURM, STROMAUSFALL,
};
use crate::languages::german::{MARS_INFO, NASA_INFO, WARNINGS};
use ggez::graphics::Color;
use ggez::Context;
use serde::{Deserialize, Serialize};
use std::sync::mpsc::Sender;
use std::time::{Duration, SystemTime};
use tracing::info;

/// defines an event which has an impact on the game and the player
/// for example a popup or a change in the player's resources
/// events can just fade off or stay as long as the player didnt interact with them
// resources
pub const DEBUG_CR: Resources<i16> = Resources {
    oxygen: 10,
    energy: 10,
    life: 0,
};
// for info events
pub const NO_CHANGE: Resources<i16> = Resources {
    oxygen: 0,
    energy: 0,
    life: 0,
};

/// Defines an event in the game
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct Event {
    name: String,
    info_text: String,
    pub(crate) resources: Resources<i16>,
    duration: Duration,
    start_time: SystemTime,
}
impl Event {
    /// create new event
    pub fn new(
        event: [&str; 2],
        sender: &Sender<StackCommand>,
        popup_message: &str,
        popup_type: &str,
        resources: Resources<i16>,
        duration: Duration,
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
            resources,
            duration,
            start_time: SystemTime::now(),
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
                DEBUG_CR,
                Duration::from_secs(10),
            )),
            11 => Some(Event::new(
                INFORMATIONSPOPUP_NASA,
                popup_sender,
                NASA_INFO[rng.usize(..4)],
                "nasa",
                NO_CHANGE,
                Duration::from_secs(10),
            )),
            22 => Some(Event::new(
                SANDSTURM,
                popup_sender,
                WARNINGS[2],
                "warning",
                DEBUG_CR,
                Duration::from_secs(10),
            )),
            33 => Some(Event::new(
                STROMAUSFALL,
                popup_sender,
                WARNINGS[1],
                "warning",
                DEBUG_CR,
                Duration::from_secs(10),
            )),
            44 => Some(Event::new(
                INFORMATIONSPOPUP_MARS,
                popup_sender,
                MARS_INFO[rng.usize(..5)],
                "mars",
                NO_CHANGE,
                Duration::from_secs(10),
            )),
            _ => None,
        }
    }
    /// Sends a popup of an event to the screen
    /// # Arguments
    /// * `popup_message` - The message which should be displayed in the popup
    /// * `sender` - The sender which is used to send the popup to the screen
    /// * `popup_type` - The type of the popup, which is used to determine the color of the popup
    /// * `event_name` - The name of the event, which is used to determine what Event name should be displayed in the popup
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
    /// Check if event is still active
    pub fn is_active(&self) -> bool {
        // check if time since event creation is greater than the duration of the event
        self.start_time.elapsed().unwrap() < self.duration
    }
    /// Returns the name of the event
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    /// Deletes due events from the gamestates events vector and adds new events
    pub fn update_events(ctx: &Context, gamestate: &mut GameState) {
        gamestate.events.retain(|event| {
            if event.is_active() {
                // event will remain in vector
                true
            } else {
                info!("Event {} is not active anymore", event.get_name());
                // the resources<i16> struct is then added to the players resources<i16>
                // removing the effect of the event
                gamestate.player.resources_change =
                    gamestate.player.resources_change + event.resources;
                // event will be removed from the events vector
                false
            }
        });
        // have a maximum of three active events
        if ctx.time.ticks() % 5000 == 0 && gamestate.events.len() < 3 {
            // generate new event
            // might not return an event
            let gen_event =
                Event::event_generator(&gamestate.screen_sender.as_ref().unwrap().clone());
            // only push events that change the change_rate of the player (at least one field is not 0)
            // ignore info events (INFORMATIONSPOPUP_NASA, INFORMATIONSPOPUP_MARS) (all their fields are 0)
            if let Some(event) = gen_event {
                if event.resources != NO_CHANGE {
                    // if the event_generator returned an event, substrack the resources<i16> struct from the players resources<i16>
                    gamestate.player.resources_change =
                        gamestate.player.resources_change - event.resources;
                    // push the event to the events vector
                    gamestate.events.push(event);
                }
            }
        }
    }
}

// TODO: Add more tests
