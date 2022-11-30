use serde::{Deserialize, Serialize};
use std::borrow::BorrowMut;
use std::fmt::{Display, Formatter};

use crate::backend::area::Area;

use crate::backend::gamestate::GameState;
use crate::game_core::item::Item;
use crate::game_core::player::Player;
use crate::game_core::resources::Resources;
use crate::languages::german::{BENZIN, GEDRUCKTESTEIL};
use crate::machines::machine::State::{Broken, Idle, Running};
use crate::machines::machine_sprite::MachineSprite;
use crate::machines::trade::Trade;
use crate::RLResult;
use ggez::graphics::{Image, Rect};
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
            Rect {
                x: 300.0,
                y: 400.0,
                w: 100.0,
                h: 50.0,
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
        interaction_area: Rect,
        trades: Vec<Trade>,
        running_resources: Resources<i16>,
    ) -> RLResult<Self> {
        info!("Creating new machine: name: {}", name);

        let sprite = MachineSprite::new(gs, name.as_str())?;
        Ok(Self {
            name,
            hit_box,
            interaction_area,
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

    fn is_trade_possible(&self, player: &Player) -> bool {
        let trade = self.get_trade();
        trade
            .cost
            .iter()
            .any(|(item, demand)| player.get_item_amount(item) >= *demand)
            && trade.resulting_state != self.state
    }
}

impl Area for Machine {
    fn interact(&mut self, player: &mut Player) -> Player {
        let t = self.get_trade();

        if !self.is_trade_possible(player) {
            return player.clone();
        }

        // all checks have been pased taking items
        info!("Executing trade:{} ", t.name);

        t.cost
            .iter()
            .for_each(|(item, demand)| player.add_item(item, -*demand));

        // If we are not in the resulting state
        match (&self.state, &t.resulting_state) {
            (Broken, Idle) | (Idle, Broken) => {}
            (Broken | Idle, Running) => {
                player.resources_change = player.resources_change + self.running_resources;
            }
            (Running, Broken | Idle) => {
                player.resources_change = player.resources_change - self.running_resources;
            }
            _ => {
                error!(
                    "Invalid Trade. Trying to change from {} to {}",
                    &self.state, &t.resulting_state
                );
            }
        }
        self.state = t.resulting_state;

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
}
