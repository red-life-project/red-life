use serde::{Deserialize, Serialize};

use crate::backend::area::Area;
use crate::backend::constants::PLAYER_INTERACTION_RADIUS;
use crate::backend::gamestate::GameState;
use crate::game_core::player::Player;
use crate::game_core::resources::Resources;
use crate::machines::machine::State::{Broken, Idle, Running};
use crate::machines::machine_sprite::MachineSprite;
use crate::machines::trade::Trade;
use crate::RLResult;
use ggez::graphics::{Image, Rect};
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
    pub fn quick(gs: &GameState) -> RLResult<Self> {
        Machine::new(
            gs,
            "test".to_string(),
            Rect {
                x: 300.0,
                y: 300.0,
                w: 100.0,
                h: 100.0,
            },
        )
    }

    pub fn new(gs: &GameState, name: String, hit_box: Rect) -> RLResult<Self> {
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
            trades: vec![],
            running_resources: Resources {
                oxygen: 0,
                energy: 0,
                life: 0,
            },
        })
    }
    pub fn no_energy(&mut self) {
        self.state = State::Idle;
        //TODO: timer pausieren
    }
}

impl Area for Machine {
    fn interact(&mut self, player: &Player) {
        match self.state {
            Broken => self.state = Idle,
            Idle => self.state = Running,
            Running => self.state = Broken,
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
