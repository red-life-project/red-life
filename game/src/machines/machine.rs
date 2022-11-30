use serde::{Deserialize, Serialize};
use std::borrow::BorrowMut;
use std::fmt::{Display, Formatter};
use std::ops::Add;
use std::sync::mpsc::Sender;

use crate::backend::area::Area;
use crate::backend::constants::PLAYER_INTERACTION_RADIUS;
use crate::backend::gamestate::GameState;
use crate::backend::rlcolor::RLColor;
use crate::backend::screen::{Popup, StackCommand};
use crate::game_core::item::Item;
use crate::game_core::player::Player;
use crate::game_core::resources::Resources;
use crate::languages::german::{BENZIN, GEDRUCKTESTEIL};
use crate::machines::machine::State::{Broken, Idle, Running};
use crate::machines::machine_sprite::MachineSprite;
use crate::machines::trade::Trade;
use crate::RLResult;
use ggez::graphics::{Color, Image, Rect};
use tracing::{error, info};
use tracing_subscriber::fmt::time;

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
    pub hit_box: Rect,
    interaction_area: Rect,
    #[serde(skip)]
    sprite: MachineSprite,
    trades: Vec<Trade>,
    running_resources: Resources<i16>,
    // sender:Sender<Resource>,
}

impl Machine {
    pub fn quick(gs: &GameState) -> RLResult<Self> {
        let clone = gs.player.inventory.clone();
        Machine::new(
            gs,
            "test".to_string(),
            Rect {
                x: 300.0,
                y: 300.0,
                w: 100.0,
                h: 100.0,
            },
            vec![
                Trade::new_and_set(
                    "repair_test".to_string(),
                    100,
                    State::Broken,
                    State::Idle,
                    &mut clone.clone(),
                    (-1, -1, -1),
                    Item::new(BENZIN),
                    0,
                ),
                Trade::new_and_set(
                    "repair_test".to_string(),
                    100,
                    State::Idle,
                    State::Running,
                    &mut clone.clone(),
                    (1, 1, 1),
                    Item::new(BENZIN),
                    0,
                ),
                Trade::new_and_set(
                    "repair_test".to_string(),
                    100,
                    State::Running,
                    State::Idle,
                    &mut clone.clone(),
                    (-2, -2, -2),
                    Item::new(GEDRUCKTESTEIL),
                    1,
                ),
            ],
            Resources {
                oxygen: 0,
                energy: -25,
                life: 0,
            },
        )
    }

    pub fn new(
        gs: &GameState,
        name: String,
        hit_box: Rect,
        trades: Vec<Trade>,
        running_resources: Resources<i16>,
    ) -> RLResult<Self> {
        info!("Creating new machine: name: {}", name);

        let sprite = MachineSprite::new(gs, name.as_str())?;
        Ok(Self {
            name,
            hit_box,
            interaction_area: Rect {
                x: hit_box.x - PLAYER_INTERACTION_RADIUS,
                y: hit_box.y - PLAYER_INTERACTION_RADIUS,
                w: hit_box.w + (PLAYER_INTERACTION_RADIUS * 2.),
                h: hit_box.h + (PLAYER_INTERACTION_RADIUS * 2.),
            },
            state: State::Broken,
            sprite,
            trades,
            running_resources,
        })
    }
    pub fn no_energy(&mut self) {
        self.state = State::Idle;
        //TODO: timer pausieren
    }
    fn get_trade(&self) -> Trade {
        // returns the first possible trade
        if let Some(t) = self.trades.iter().find(|t| t.initial_state == self.state) {
            return t.clone();
        }
        Trade::default()
    }
}

impl Area for Machine {
    fn interact(&mut self, player: &mut Player, sender: &Sender<StackCommand>) -> Player {
        let t = self.get_trade();
        if t.cost
            .iter()
            .any(|(item, demand)| player.get_item_amount(item) - demand < 0)
        {
            let popup = Popup::new(RLColor::RED, "Fehlende Items".to_string(), 5);
            info!("Popup for Trade conflict sent");
            sender.send(StackCommand::Popup(popup)).unwrap();
            return player.clone();
        }

        // all checks have been pased taking items
        info!("Executing trade:{} ", t.name);

        &t.cost
            .iter()
            .for_each(|(item, demand)| player.add_item(item, *demand * -1));

        /*     match self.state {
            // generalisation
            Broken => self.state = Idle,
            Idle => self.state = Running,
            Running => self.state = Running,
        };*/
        if self.state != t.resulting_state {
            // if the state changed
            match (&self.state, &t.resulting_state) {
                (Broken, Idle) | (Idle, Broken) => {}
                (Broken, Running) | (Idle, Running) => {
                    player.resources_change = player.resources_change + self.running_resources;
                }
                (Running, Broken) | (Running, Idle) => {
                    player.resources_change = player.resources_change - self.running_resources;
                }
                _ => {
                    error!(
                        "unexpected case in Match. mashiene state changed from {} to {}",
                        &self.state, &t.resulting_state
                    )
                }
            }
            self.state = t.resulting_state;
        }

        player.clone()
    }

    fn get_collision_area(&self) -> Rect {
        self.hit_box
    }

    fn get_interaction_area(&self) -> Rect {
        self.interaction_area
    }

    fn get_graphic(&self) -> Image {
        match self.state {
            Broken => self.sprite.broken.clone(),
            Idle => self.sprite.idle.clone(),
            Running => self.sprite.running.clone(),
        }
    }

    fn is_non_broken_machine(&self) -> bool {
        self.state != Broken
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn get_state(&self) -> State {
        self.state.clone()
    }
}
