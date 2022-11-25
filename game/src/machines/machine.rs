use serde::{Deserialize, Serialize};

use crate::backend::area::Area;

use crate::backend::gamestate::GameState;
use crate::game_core::player::Player;
use crate::game_core::resources::Resources;
use crate::machines::machine::State::{Broken, Idel, Running};
use crate::machines::machine_sprite::MaschineSprite;
use crate::machines::trade::Trade;
use crate::RLResult;
use ggez::graphics::{Image, Rect};
use std::sync::mpsc::Sender;
use tracing::info;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum State {
    Broken,
    Idel,
    Running,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mashine {
    pub name: String,
    pub state: State,
    hitbox: Rect,
    interaction_area: Rect,
    #[serde(skip)]
    sprite: MaschineSprite,
    trades: Vec<Trade>,
    running_recources: Resources<i16>,
    // sender:Sender<Resource>,
}

impl Mashine {
    pub fn default(gs: &GameState) -> Self {
        Mashine::new(
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

    pub fn new(gs: &GameState, name: String, hitbox: Rect, interaction_area: Rect) -> Self {
        info!("Creating new machine: name: {}", name);

        let sprite = MaschineSprite::new(gs, name.as_str());
        Self {
            name,
            hitbox,
            interaction_area,
            state: State::Broken,
            sprite,
            trades: vec![],
            running_recources: Resources {
                oxygen: 0,
                energy: 0,
                life: 0,
            }, //  sender: ()
        }
    }
    pub fn no_energy(&mut self) {
        self.state = State::Idel;
        //timer pausieren
    }
}

impl Area for Mashine {
    fn interact(&mut self, player: &Player) {
        match self.state {
            Broken => self.state = Idel,
            State::Idel => self.state = Running,
            State::Running => self.state = Broken,
        }
    }

    fn get_collision_area(&self) -> Rect {
        self.hitbox
    }

    fn get_interaction_area(&self) -> Rect {
        self.interaction_area
    }

    fn get_graphic(&self) -> Image {
        match self.state {
            Broken => self.sprite.broken.clone(),
            State::Idel => self.sprite.idel.clone(),
            State::Running => self.sprite.running.clone(),
        }
    }

    fn is_non_broken_maschien(&self) -> bool {
        return self.state != Broken;
    }

    fn get_name(&self) -> String {
        return self.name.clone();
    }
}
