use serde::{Deserialize, Serialize};

use crate::backend::area::Area;
use crate::backend::gamestate::GameState;
use crate::game_core::player::Player;
use crate::game_core::resources::Resources;
use crate::machines::machine_sprite::MaschineSprite;
use crate::machines::trade::Trade;
use ggez::graphics::Rect;
use serde_yaml::Value::Null;
use std::ptr::null;
use std::sync::mpsc::Sender;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum State {
    Broken,
    Idel,
    Running,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Maschine {
    //gamestate:GameState,
    name: String,
    hitbox: Rect,

    interaction_area: Rect,
    state: State,
    sprite: MaschineSprite,
    trades: Vec<Trade>,
    running_recources: Resources<i16>,
    // sender:Sender<Resource>,
}
impl Default for Maschine {
    fn default() -> Self {
        Self {
            //gamestate:GameState::default(),
            name: "Maschine ohne namen".to_string(),
            hitbox: Rect::default(),
            interaction_area: Rect::default(),
            state: State::Broken,
            sprite: MaschineSprite::default(),
            trades: vec![],
            running_recources: Resources::default(), //  sender: ()
        }
    }
}

impl Maschine {
    pub fn test_maschine(/*gs:GameState*/) -> Maschine {
        //let msSprite =  get_asset("player.png")?;

        Self {
            //gamestate:gs,
            name: "test_Maschiene".to_string(),
            hitbox: Rect {
                x: 300.0,
                y: 300.0,
                w: 100.0,
                h: 100.0,
            },
            interaction_area: Rect {
                x: 300.0,
                y: 400.0,
                w: 100.0,
                h: 50.0,
            },
            state: State::Broken,
            sprite: Default::default(),
            trades: vec![],
            running_recources: Resources::default(),
        }
    }
    /*
    pub fn new(/*gs:GameState,*/namen: String, trades: Vec<Trade>) -> Maschine {

        //let loadedSprite: MaschineSprite =  AssetService::get(name);
        //let loded = GameState

        Self {
           // gamestate: gs,
            name: namen,
            hitbox: Default::default(),
            interaction_area: Default::default(),
            state: State::Broken,
            sprite: Default::default(),
            trades,
            running_recources: Resources::default(),
         //   sender: ()
        }
    }
    */

    pub fn no_energy(&mut self) {
        self.state = State::Idel;
        //timer pausiren
    }
}

impl Area for Maschine {
    fn interact(&mut self, player: &Player) {
        todo!()
    }

    fn get_collision_area(&self) -> Rect {
        todo!()
    }

    fn get_interaction_area(&self) -> Rect {
        todo!()
    }
}
