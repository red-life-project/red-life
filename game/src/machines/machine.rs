use serde::{Deserialize, Serialize};
use std::borrow::BorrowMut;

use crate::backend::area::Area;

use crate::backend::gamestate::GameState;
use crate::game_core::item::{Item, BENZIN, GEDRUCKTESTEIL};
use crate::game_core::player::Player;
use crate::game_core::resources::Resources;
use crate::machines::machine::State::{Broken, Idle, Running};
use crate::machines::machine_sprite::MachineSprite;
use crate::machines::trade::Trade;
use crate::RLResult;
use ggez::graphics::{Image, Rect};
use std::sync::mpsc::Sender;
use tracing::info;
use tracing_subscriber::fmt::time;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum State {
    Broken,
    Idle,
    Running,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Machine {
    pub name: String,
    pub state: State,
    hit_box: Rect,
    interaction_area: Rect,
    #[serde(skip)]
    sprite: MachineSprite,
    trades: Vec<Trade>,
    running_resources: Resources<i16>,
    // sender:Sender<Resource>,
}

impl Machine {
    pub fn quick(gs: &GameState) -> Self {
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
                    clone.clone(),
                    (0, 0, 0),
                    Item::new(BENZIN),
                    0,
                ),
                Trade::new_and_set(
                    "repair_test".to_string(),
                    100,
                    State::Idle,
                    clone.clone(),
                    (-10, -10, 0),
                    Item::new(BENZIN),
                    0,
                ),
                Trade::new_and_set(
                    "repair_test".to_string(),
                    100,
                    State::Running,
                    clone,
                    (1, 1, 0),
                    Item::new(GEDRUCKTESTEIL),
                    1,
                ),
            ],
        )
    }

    pub fn new(
        gs: &GameState,
        name: String,
        hit_box: Rect,
        interaction_area: Rect,
        trades: Vec<Trade>,
    ) -> Self {
        info!("Creating new machine: name: {}", name);

        let sprite = MachineSprite::new(gs, name.as_str());
        Self {
            name,
            hit_box,
            interaction_area,
            state: State::Broken,
            sprite,
            trades,
            running_resources: Resources {
                oxygen: 0,
                energy: 0,
                life: 0,
            }, //  sender: ()
        }
    }
    pub fn no_energy(&mut self) {
        self.state = State::Idle;
        //timer pausieren
    }
    fn get_trade(&self) -> Trade {
        // returns the first possible trade
        if let Some(t) = self
            .trades
            .iter()
            .filter(|t| t.ms_state == self.state)
            .next()
        {
            return t.clone();
        }
        Trade::default()
    }
}

impl Area for Machine {
    fn interact(&mut self, mut player_inventory: Vec<(Item, i32)>) -> Vec<(Item, i32)> {
        let t = self.get_trade();
        for demand in &t.cost {
            for supply in &player_inventory {
                if supply.0.name == demand.0.name && supply.1 - demand.1 < 0 {
                    //if there is one item in the list of demands the interaction fails nothing changes
                    //TODO: inform player about failed interaction
                    return player_inventory.clone();
                }
            }
        }
        // all checks have been pased taking items
        info!("Executing trade:{} ", t.name);
        for demand in &t.cost {
            for supply in player_inventory.borrow_mut() {
                if supply.0.name == demand.0.name {
                    supply.1 -= demand.1
                }
            }
        }
        match self.state {
            // generalisation
            Broken => self.state = Idle,
            Idle => self.state = Running,
            Running => self.state = Running,
        };
        player_inventory.clone()
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
            State::Idle => self.sprite.idle.clone(),
            State::Running => self.sprite.running.clone(),
        }
    }

    fn is_non_broken_machine(&self) -> bool {
        return self.state != Broken;
    }

    fn get_name(&self) -> String {
        return self.name.clone();
    }
}
