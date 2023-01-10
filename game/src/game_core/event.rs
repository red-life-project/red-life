use crate::backend::constants::{ObjectId, PopupType, DESIRED_FPS, SANDSTORM_CR, EventId};
use crate::backend::gamestate::GameState;
use crate::backend::screen::{Popup, ScreenCommand, StackCommand};
use crate::game_core::resources::Resources;
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
    event_id: EventId,
    pub(crate) resources: Option<Resources<i16>>,
    duration: u32,
    popup_type: PopupType,
    message_id: usize,
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
        event_id: EventId,
        popup_type: PopupType,
        resources: Option<Resources<i16>>,
        duration: u32,
        message_id: usize,
    ) -> Self {
        info!(
            "New event created: {:?}",
            event_id
        );
        Self {
            event_id,
            resources,
            duration: duration * DESIRED_FPS,
            popup_type,
            message_id,
        }
    }

    /// if no Event is active it either chooses a random event of the Event enum or nothing every 60 seconds
    #[allow(clippy::pedantic)]
    pub fn event_generator() -> Option<Event> {
        let rng = fastrand::Rng::new();
        match rng.usize(..15) {
            8 => Some(Event::new(
                EventId::Sandstorm,
                PopupType::Warning,
                Some(SANDSTORM_CR),
                5,
                0
            )),
            0 | 3 => Some(Event::new(
                EventId::CometStrike,
                PopupType::Warning,
                None,
                0,
                0
            )),
            1 => Some(Event::new(
                EventId::InformationPopupNasa,
                PopupType::Nasa,
                None,
                0,
                rng.usize(..4)
            )),
            2 | 9 | 7 => Some(Event::new(
                EventId::PowerFailure,
                PopupType::Warning,
                None,
                0,
                0
            )),
            4 => Some(Event::new(
                EventId::InformationPopupMars,
                PopupType::Mars,
                None,
                0,
                rng.usize(..5)
            )),
            _ => None,
        }
    }

    /// Sends a popup of an event to the screen
    /// # Arguments
    /// * `popup_message` - The message which should be displayed in the popup
    /// * `sender` - The sender which is used to send the popup to the screen
    /// * `popup_type` - The type of the popup, which is used to determine the color of the popup
    /// * `event_id` - The id of the event, which is used to determine what Event name should be displayed in the popup
    pub fn send_popup(
        popup_message: &'static str,
        sender: &Sender<StackCommand>,
        popup_type: PopupType,
        event_id: EventId,
    ) -> RLResult {
        let popup = match popup_type {
            PopupType::Warning => Popup::warning(popup_message.into()),
            PopupType::Nasa => Popup::nasa(popup_message.into()),
            PopupType::Mars => Popup::mars(popup_message.into()),
        };
        sender.send(StackCommand::Screen(ScreenCommand::Popup(popup)))?;
        info!(
            "Event Popup sent: name: {:?}, Popup-Message: {}, Popup-Type: {:?}",
            event_id,
            popup_message,
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
    /// * `game_state` - The game state which is used to access the player and the machines
    pub fn action(&self, restore: bool, game_state: &mut GameState) -> RLResult {
        let sender = game_state.get_screen_sender()?.clone();

        // handle event effects
        match self.event_id {
            EventId::CometStrike => {
                if let Some(one_hole) = game_state
                    .machines
                    .iter_mut()
                    .find(|machine| machine.id.is_hole() && machine.state != State::Running)
                {
                    // event not triggered if both machine are already running
                    Event::send_popup(&self.event_id.warning(game_state.lng, self.message_id), &sender, self.popup_type, self.event_id)
                        .unwrap();
                    one_hole.change_state_to(&State::Running);
                }
            }
            EventId::PowerFailure => {
                game_state.machines.iter_mut().for_each(|machine| {
                    // if machine is running it will b use tracing::{info, Id};e stopped
                    // event not triggered if machine is broken or idling
                    if machine.id == ObjectId::PowerGenerator && machine.state == State::Running {
                        Event::send_popup(
                            &self.event_id.warning(game_state.lng, self.message_id),
                            &sender,
                            self.popup_type,
                            self.event_id,
                        )
                        .unwrap();
                        machine.change_state_to(&State::Idle);
                    }
                });
            }
            // apply direct resource changes if there are any and the event is not handled above
            _ => {
                Event::send_popup(&self.event_id.warning(game_state.lng, self.message_id), &sender, self.popup_type, self.event_id)?;
                if let Some(resources) = self.resources {
                    if restore {
                        game_state.player.resources_change =
                            game_state.player.resources_change + resources;
                    } else {
                        game_state.player.resources_change =
                            game_state.player.resources_change - resources;
                    }
                }
            }
        }
        info!("Event triggered (restore: {}): {:?}", restore, self.event_id);
        Ok(())
    }

    /// Deletes due events from the game states events vector and adds new events
    /// # Arguments
    /// * `game_state` - The game state which is used to access the events vector
    /// * `context` - The game context which is used to access the current tick
    pub fn update_events(ctx: &Context, game_state: &mut GameState) -> RLResult {
        if ctx.time.ticks() % 20 == 0 {
            game_state.events.iter_mut().for_each(|event| {
                event.duration = event.duration.saturating_sub(20);
                if event.event_id == EventId::Sandstorm {}
            });
            // restore resources of inactive events
            for event in &game_state.events {
                if !event.is_active() {
                    if let Some(resources) = event.resources {
                        game_state.player.resources_change =
                            game_state.player.resources_change + resources;
                    }
                }
            }
            // remove all events which are not active anymore
            game_state.events.retain(|event| {
                if event.is_active() {
                    true
                } else {
                    info!("Event {:?} is not active anymore", event.event_id);
                    false
                }
            });
        }
        // have a maximum of one active event
        if ctx.time.ticks() >= 400 && ctx.time.ticks() % 200 == 0 {
            // generate new event
            // might not return an event
            let gen_event = Event::event_generator();
            // if event is not none, add it to the game states events vector and activate apply its effect
            if let Some(event) = gen_event {
                event.action(false, game_state)?;
                game_state.events.push(event);
            }
        }
        Ok(())
    }
}
