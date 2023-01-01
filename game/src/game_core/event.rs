use crate::backend::constants::{ObjectId, PopupType, DESIRED_FPS, SANDSTORM_CR};
use crate::backend::gamestate::GameState;
use crate::backend::screen::{Popup, StackCommand};
use crate::game_core::resources::Resources;
use crate::languages::{
    comet_strike, informations_popup_mars, informations_popup_nasa, mars_info, nasa_info,
    power_failure, sandstorm, warnings, Lang,
};
use crate::machines::machine::State;
use crate::RLResult;
use ggez::Context;
use serde::{Deserialize, Serialize};
use std::sync::mpsc::Sender;
use tracing::info;

/// defines an event which has an impact on the game and the player
/// for example a popup or a change in the player's resources
/// events can just fade off or stay as long as the player didnt interact with them

/// Defines an event in the game
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub(crate) struct Event {
    name: String,
    info_text: String,
    pub(crate) resources: Option<Resources<i16>>,
    duration: u32,
    popup_type: PopupType,
    popup_message: String,
}

impl Event {
    /// create new event
    /// # Arguments
    /// * `event` - name and info text of the event
    /// * `resources` - resources which are affected by the event
    /// * `duration` - duration of the event in seconds
    /// * `popup_type` - type of the popup which is shown when the event starts
    /// * `popup_message` - message of the popup which is shown when the event starts
    pub fn new(
        event: [&str; 2],
        popup_message: &str,
        popup_type: PopupType,
        resources: Option<Resources<i16>>,
        duration: u32,
    ) -> Self {
        info!(
            "New event created: {}, info text: {}",
            event[0].to_string(),
            event[1].to_string()
        );
        Self {
            name: event[0].to_string(),
            info_text: event[1].to_string(),
            resources,
            duration: duration * DESIRED_FPS,
            popup_type,
            popup_message: popup_message.to_string(),
        }
    }

    /// if no Event is active it either chooses a random event of the Event enum or nothing every 60 seconds
    #[allow(clippy::pedantic)]
    pub fn event_generator(lng: Lang) -> Option<Event> {
        let rng = fastrand::Rng::new();
        match rng.usize(..15) {
            8 => Some(Event::new(
                *sandstorm(lng),
                warnings(lng)[2],
                PopupType::Warning,
                Some(SANDSTORM_CR),
                5,
            )),
            0 | 3 => Some(Event::new(
                *comet_strike(lng),
                warnings(lng)[0],
                PopupType::Warning,
                None,
                0,
            )),
            1 => Some(Event::new(
                *informations_popup_nasa(lng),
                nasa_info(lng)[rng.usize(..4)],
                PopupType::Nasa,
                None,
                0,
            )),
            2 | 9 | 7 => Some(Event::new(
                *power_failure(lng),
                warnings(lng)[1],
                PopupType::Warning,
                None,
                0,
            )),
            4 => Some(Event::new(
                *informations_popup_mars(lng),
                mars_info(lng)[rng.usize(..5)],
                PopupType::Mars,
                None,
                0,
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
        popup_type: PopupType,
        event_name: &str,
    ) -> RLResult {
        let popup = match popup_type {
            PopupType::Warning => Popup::warning(popup_message.to_string()),
            PopupType::Nasa => Popup::nasa(popup_message.to_string()),
            PopupType::Mars => Popup::mars(popup_message.to_string()),
            // _ => Popup::new(Color::RED, "Error".to_string(), 10),
        };
        sender.send(StackCommand::Popup(popup))?;
        info!(
            "Event Popup sent: name: {}, Popup-Message: {}, Popup-Type: {:?}",
            event_name,
            popup_message.to_string(),
            popup_type
        );
        Ok(())
    }

    /// Check if event is still active
    pub fn is_active(&self) -> bool {
        self.duration != 0
    }

    /// Triggers the event and activates its effect
    /// # Arguments
    /// * `restore` - If true the event will be deactivated and the resources will be restored
    /// * `gamestate` - The gamestate which is used to access the player and the machines
    pub fn action(&self, restore: bool, gamestate: &mut GameState) -> RLResult {
        let lng = gamestate.lng;
        let comet_strike: &str = comet_strike(lng)[0];
        let power_failure: &str = power_failure(lng)[0];
        let sender = gamestate.get_screen_sender()?.clone();

        // handle event effects
        match self.name.as_str() {
            s if comet_strike == s => {
                if let Some(one_hole) = gamestate
                    .machines
                    .iter_mut()
                    .find(|machine| machine.id.is_hole() && machine.state != State::Running)
                {
                    // event not triggered if both machine are already running
                    Event::send_popup(&self.popup_message, &sender, self.popup_type, &self.name)
                        .unwrap();
                    one_hole.change_state_to(&State::Running);
                }
            }
            s if s == power_failure => {
                gamestate.machines.iter_mut().for_each(|machine| {
                    // if machine is running it will b use tracing::{info, Id};e stopped
                    // event not triggered if machine is broken or idling
                    if machine.id == ObjectId::PowerGenerator && machine.state == State::Running {
                        Event::send_popup(
                            &self.popup_message,
                            &sender,
                            self.popup_type,
                            &self.name,
                        )
                        .unwrap();
                        machine.change_state_to(&State::Idle);
                    }
                });
            }
            // apply direct resource changes if there are any and the event is not handled above
            _ => {
                Event::send_popup(&self.popup_message, &sender, self.popup_type, &self.name)?;
                if let Some(resources) = self.resources {
                    if restore {
                        gamestate.player.resources_change =
                            gamestate.player.resources_change + resources;
                    } else {
                        gamestate.player.resources_change =
                            gamestate.player.resources_change - resources;
                    }
                }
            }
        }
        info!("Event triggered (restore: {}): {}", restore, self.name);
        Ok(())
    }

    /// Returns the name of the event
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Deletes due events from the gamestates events vector and adds new events
    /// # Arguments
    /// * `gamestate` - The gamestate which is used to access the events vector
    /// * `context` - The game context which is used to access the current tick
    pub fn update_events(ctx: &Context, gamestate: &mut GameState) -> RLResult {
        let lng = gamestate.lng;
        if ctx.time.ticks() % 20 == 0 {
            gamestate.events.iter_mut().for_each(|event| {
                event.duration = event.duration.saturating_sub(20);
                if event.name == "Sandsturm" {}
            });
            // restore resources of inactive events
            for event in &gamestate.events {
                if !event.is_active() {
                    if let Some(resources) = event.resources {
                        gamestate.player.resources_change =
                            gamestate.player.resources_change + resources;
                    }
                }
            }
            // remove all events which are not active anymore
            gamestate.events.retain(|event| {
                if event.is_active() {
                    true
                } else {
                    info!("Event {} is not active anymore", event.get_name());
                    false
                }
            });
        }
        // have a maximum of one active event
        if ctx.time.ticks() >= 400 && ctx.time.ticks() % 200 == 0 {
            // generate new event
            // might not return an event
            let gen_event = Event::event_generator(lng);
            // if event is not none, add it to the gamestates events vector and activate apply its effect
            if let Some(event) = gen_event {
                event.action(false, gamestate)?;
                gamestate.events.push(event);
            }
        }
        Ok(())
    }
}
