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
use crate::machines::machine::State::{Broken, Idel, Running};
use crate::RLResult;

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
    pub fn default(gs: &GameState) -> Self
    {
        Mashine::new(gs,
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
                     },)
    }

    pub fn new(gs: &GameState, name: String,hitbox:Rect,interaction_area:Rect) -> Self {
        // let sprite = Some(gs.get_asset( "test_mashine.png").unwrap().clone());
        let sprite = MaschineSprite::new(gs, name.as_str());
        //let test : &Sender<Resources<i16>> = GameState::
        Self {

            //gamestate:GameState::default(),
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
    /*
        pub fn test_mashine(gs :&GameState) -> Mashine {
            //let msSprite =  get_asset("player.png")?;
            let sprite =
                Some(
                    gs.get_asset( "test_mashine.png")
                        .unwrap()
                        .clone()


                );
           //("Check if sprite is none");


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

                trades: vec![],
                running_recources: Resources {
                    oxygen: 0,
                    energy: 0,
                    life: 0,
                },
                sprite: Default::default()
            }
        }

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
        match self.state {
            Broken => { self.state = Idel}
            State::Idel => {self.state= Running}
            State::Running => { self.state = Broken}
        }
    }

    fn get_collision_area(&self) -> Rect {
        return self.hitbox;
    }

    fn get_interaction_area(&self) -> Rect {
        return self.interaction_area;
    }

    fn get_graphic(&self) -> Image {
        //TODO:
        // switch case
        // if state is a b c
        // return maschinen sprite.a .b .c

        match self.state {
            Broken => {self.sprite.broken.clone()}
            State::Idel => {self.sprite.idel.clone()}
            State::Running => { self.sprite.running.clone()}
        }
    }

    fn check(&self) -> bool {
        return self.state != Broken;
    }
}
