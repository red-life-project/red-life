use serde::{Deserialize, Serialize};

use crate::backend::area::Area;
use crate::backend::gamestate::GameState;
use crate::game_core::player::Player;
use crate::game_core::resources::Resources;
use crate::machines::machine_sprite::MaschineSprite;
use crate::machines::trade::Trade;
use ggez::graphics::{Image, Rect};
use serde_yaml::Value::Null;
use std::ptr::null;
use std::sync::mpsc::Sender;
use ggez::winit::event::VirtualKeyCode::G;
use crate::RLResult;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum State {
    Broken,
    Idel,
    Running,
}

#[derive(Debug, Clone)]
pub struct Mashine {
    //gamestate:GameState,
    pub name: String,
    hitbox: Rect,
    interaction_area: Rect,
    //sprite: MaschineSprite,
    test: Image,
    pub state: State,
    sprite: MaschineSprite,
    trades: Vec<Trade>,
    running_recources: Resources<i16>,
    // sender:Sender<Resource>,
}
impl Mashine{
    fn default(gs:GameState) -> Self {
       let mut sprite: Image = gs.get_asset( "test_Maschiene").unwrap().clone();
        //let test : &Sender<Resources<i16>> = GameState::
        Self {

            //gamestate:GameState::default(),
            name: "Maschine ohne namen".to_string(),
            hitbox: Rect::default(),
            interaction_area: Rect::default(),
            state: State::Broken,
           // sprite: MaschineSprite::default(),
            test: sprite,
            trades: vec![],
            running_recources: Resources {
                oxygen: 0,
                energy: 0,
                life: 0,
            }, //  sender: ()
        }
    }

    pub fn test_mashine(gs :&GameState) -> Mashine {
        //let msSprite =  get_asset("player.png")?;
        let sprite: Image = gs.get_asset( "test_Maschiene").unwrap().clone();
        todo!("Check if sprite is none");


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
            //sprite: MaschineSprite::default(),
            test: sprite,
            trades: vec![],
            running_recources: Resources {
                oxygen: 0,
                energy: 0,
                life: 0,
            },
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
        //timer pausieren
    }
}

impl Area for Mashine {
    fn interact(&mut self, player: &Player) {
        todo!()
    }

    fn get_collision_area(&self) -> Rect {
        return self.hitbox;
    }

    fn get_interaction_area(&self) -> Rect {
        return self.interaction_area;
    }

    fn get_graphic(&self) -> &Image {
        todo!();
        //TODO:
        // switch case
        // if state is a b c
        // return maschinen sprite.a .b .c
        return &self.test
    }
}
