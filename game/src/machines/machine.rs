use crate::backend::constants::PLAYER_INTERACTION_RADIUS;
use crate::backend::gamestate::GameCommand;
use crate::backend::rlcolor::RLColor;
use crate::backend::screen::{Popup, StackCommand};
use crate::backend::utils::is_colliding;
use crate::game_core::item::Item;
use crate::game_core::player::Player;
use crate::game_core::resources::Resources;
use crate::languages::german::TRADE_CONFLICT_POPUP;
use crate::machines::machine::State::{Broken, Idle, Running};
use crate::machines::machine_sprite::MachineSprite;
use crate::machines::trade::Trade;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::sync::mpsc::Sender;

use crate::RLResult;
use ggez::graphics::{Color, Image, Rect};
use tracing::info;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum State {
    Broken,
    Idle,
    Running,
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Broken => write!(f, "Broken"),
            Idle => write!(f, "Idle"),
            Running => write!(f, "Running"),
        }
    }
}

impl From<State> for Color {
    fn from(value: State) -> Self {
        match value {
            Broken => RLColor::STATUS_RED,
            Idle => RLColor::STATUS_YELLOW,
            Running => RLColor::STATUS_GREEN,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Machine {
    pub name: String,
    pub state: State,
    pub hitbox: Rect,
    pub interaction_area: Rect,
    pub trades: Vec<Trade>,
    last_trade: Trade,
    running_resources: Resources<i16>,
    og_time: i16,
    time_remaining: i16,
    time_change: i16,
    #[serde(skip)]
    sprite: Option<MachineSprite>,
    #[serde(skip)]
    sender: Option<Sender<GameCommand>>,
    #[serde(skip)]
    screen_sender: Option<Sender<StackCommand>>,
}

impl Machine {
    pub(crate) fn is_interactable(&self, pos: (usize, usize)) -> bool {
        is_colliding(pos, &self.interaction_area)
    }
    pub fn new_by_const(
        (name, hit_box, trades, running_resources): (String, Rect, Vec<Trade>, Resources<i16>),
    ) -> Self {
        Machine::new(name, hit_box, trades, running_resources)
    }

    /// Loads the Machine Sprites. Has to be called before drawing.
    pub(crate) fn init(
        &mut self,
        images: &[Image],
        sender: Sender<GameCommand>,
        screen_sender: Sender<StackCommand>,
    ) {
        self.sprite = Some(images.into());
        self.sender = Some(sender);
        self.screen_sender = Some(screen_sender);
        if self.name == "Loch" {
            if (self.hitbox.x - 780.).abs() == 0_f32 {
                // constant
                self.change_state_to(&Running);
            } else {
                self.change_state_to(&Idle);
            }
        }
    }

    fn new(
        // this function is supposed to be private
        name: String,
        hit_box: Rect,
        trades: Vec<Trade>,
        running_resources: Resources<i16>,
    ) -> Self {
        info!("Creating new machine: name: {}", name);

        //let sprite = MachineSprite::new(gs, name.as_str())?;
        Self {
            name,
            hitbox: hit_box,
            interaction_area: Rect {
                x: hit_box.x - PLAYER_INTERACTION_RADIUS,
                y: hit_box.y - PLAYER_INTERACTION_RADIUS,
                w: hit_box.w + (PLAYER_INTERACTION_RADIUS * 2.),
                h: hit_box.h + (PLAYER_INTERACTION_RADIUS * 2.),
            },
            state: Broken,
            sprite: None,
            trades,
            last_trade: Trade::default(),
            running_resources,
            og_time: 0,
            time_remaining: 0,
            time_change: 0,
            sender: None,
            screen_sender: None,
        }
    }
    fn get_trade(&self) -> Trade {
        // returns the first possible trade
        if let Some(t) = self.trades.iter().find(|t| t.initial_state == self.state) {
            return t.clone();
        }
        Trade::default()
    }

    fn check_change(&self, before: &State, after: &State) {
        match (before, after) {
            (Broken, Idle) => {
                let _e = self.sender.as_ref().unwrap().send(GameCommand::Milestone);
            }
            (Idle, Broken) => {}
            (Broken | Idle, Running) => {
                let _e = self
                    .sender
                    .as_ref()
                    .unwrap()
                    .send(GameCommand::ResourceChange(self.running_resources));
                let _e = self.sender.as_ref().unwrap().send(GameCommand::Milestone);
            }
            (Running, Broken | Idle) => {
                let _e = self
                    .sender
                    .as_ref()
                    .unwrap()
                    .send(GameCommand::ResourceChange(
                        // 0-n = n*-1  = n.invert()                            // TODO: add .invert() to Resources
                        Resources {
                            oxygen: 0,
                            energy: 0,
                            life: 0,
                        } - self.running_resources,
                    ));
            }
            _ => {
                info!(
                    "unexpected case in Match. machine state changed from {} to {}",
                    before.clone(),
                    after.clone()
                );
            }
        }
    }

    pub(crate) fn change_state_to(&mut self, new_state: &State) {
        if self.state != *new_state {
            self.check_change(&self.state, new_state);
            self.state = new_state.clone();
        }
    }

    pub(crate) fn interact(&mut self, player: &mut Player) -> RLResult<Player> {
        let trade = self.get_trade();
        if trade.name == *"no_Trade" {
            return Ok(player.clone());
        }
        if player.resources.energy == 0 && self.running_resources.energy < 0 && self.name != "Loch"
        {
            return Ok(player.clone());
        }

        // dif = items the player has - the cost of the trade
        let dif = trade
            .cost
            .iter()
            .map(|(item, demand)| (item, player.get_item_amount(item) - demand))
            .filter(|(_item, dif)| *dif < 0)
            .collect::<Vec<(&Item, i32)>>();
        // If one item is not available in enough quantity
        if dif.iter().any(|(_, demand)| *demand < 0) {
            let mut missing_items = String::new();
            dif.iter()
                .map(|(item, amount)| format!("*{} {}\n", amount * -1, item.name))
                .for_each(|x| missing_items.push_str(&x));
            let popup = Popup::info(format!("{}\n{missing_items}", TRADE_CONFLICT_POPUP[0]));
            info!(
                "Popup for Trade conflict sent: Missing Items: {}",
                missing_items
            );
            self.screen_sender
                .as_ref()
                .unwrap()
                .send(StackCommand::Popup(popup))?;
            return Ok(player.clone());
        }

        // the player has enough items for the trade so we will execute on it
        info!("Executing trade:{} ", trade.name);

        // Remove the cost of the trade from the players inventory by sending the demand to the AddItem GameCommand
        let items_cost = trade
            .cost
            .iter()
            .filter(|(_, demand)| *demand >= 0)
            .map(|(item, demand)| (item.clone(), -*demand))
            .collect::<Vec<(Item, i32)>>();
        self.sender
            .as_ref()
            .unwrap()
            .send(GameCommand::AddItems(items_cost))
            .expect("could not send AddItems");

        if trade.time_ticks == 0 {
            // this trade has no timer
            self.time_change = 0;
        } else {
            //this trade has a timer
            if self.time_remaining == 0 {
                //if no timer is running set timer up
                self.last_trade = trade.clone();
                self.time_remaining = trade.time_ticks;
                self.og_time = trade.time_ticks;
            }
            //start the timer
            self.time_change = 1;
        }

        if trade.return_after_timer {
            self.change_state_to(&trade.resulting_state);
        }

        Ok(player.clone())
    }

    pub(crate) fn get_graphic(&self) -> &Image {
        self.sprite.as_ref().unwrap().get(self.state.clone())
    }

    pub(crate) fn tick(&mut self, delta_ticks: i16) -> RLResult {
        self.time_remaining -= self.time_change * delta_ticks;
        if self.time_remaining < 0 {
            //timer run out
            self.time_change = 0;
            self.time_remaining = 0;

            if self.last_trade.return_after_timer {
                if self.last_trade.name == "Notfall_signal_absetzen" {
                    let _e = self.sender.as_ref().unwrap().send(GameCommand::Winning);
                }

                self.change_state_to(&self.last_trade.initial_state.clone());
            } else {
                self.change_state_to(&self.last_trade.resulting_state.clone());
            }
            // After Trade ended, send the GameCommand AddItems to add the earning Items to the players inventory
            let trade = self.last_trade.clone();
            let items = trade
                .cost
                .iter()
                .filter(|(_, demand)| *demand < 0)
                .map(|(item, demand)| (item.clone(), -*demand))
                .collect::<Vec<(Item, i32)>>();
            self.sender
                .as_ref()
                .unwrap()
                .send(GameCommand::AddItems(items))?;
        }
        Ok(())
    }
    pub(crate) fn get_time_percentage(&self) -> f32 {
        if self.og_time == 0 {
            -1.0
        } else {
            f32::from(self.time_remaining) / f32::from(self.og_time)
        }
    }
    pub fn no_energy(&mut self) {
        if self.running_resources.energy < 0 && self.name != "Loch" {
            // if the is no energy and the machine needs some we stop it
            if self.state == Running {
                self.change_state_to(&Idle);
                self.time_change = 0;
            }
        }
    }
}
