use serde::{Deserialize, Serialize};

use crate::backend::area::Area;

use crate::backend::gamestate::GameState;
use crate::game_core::player::Player;
use crate::game_core::resources::Resources;
use crate::machines::machine::State::{Broken, Idle, Running};
use crate::machines::machine_sprite::MachineSprite;
use crate::machines::trade::Trade;
use crate::RLResult;
use ggez::graphics::{Image, Rect};
use std::sync::mpsc::Sender;
use tracing::info;

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
        )
    }

    pub fn new(gs: &GameState, name: String, hit_box: Rect, interaction_area: Rect) -> Self {
        info!("Creating new machine: name: {}", name);

        let sprite = MachineSprite::new(gs, name.as_str());
        Self {
            name,
            hit_box,
            interaction_area,
            state: State::Broken,
            sprite,
            trades: vec![],
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
}

impl Area for Machine {
    fn interact(&mut self, player: &Player) {
        match self.state {
            Broken => self.state = Idle,
            State::Idle => self.state = Running,
            State::Running => self.state = Broken,
        }
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
