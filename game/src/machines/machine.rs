use serde::{Deserialize, Serialize};

use std::default::default;
use std::ptr::null;
use std::sync::mpsc::Sender;
use ggez::graphics::Rect;
use crate::backend::area::Area;
use crate::basis::ressourcen::Resource;
use crate::game_core::player::Player;
use crate::machines::machine_sprite::MaschineSprite;

pub enum State {
    Broken,
    Idel,
    Running
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Maschine {
    name: String,
    hitbox: Rect,

    interaction_area: Rect,
    state: State,
    sprite: MaschineSprite,
    trades: Vec<Trade>,
    running_recources : Resource,
    sender:Sender<Resource>;

}
impl Default for Maschine
{
    fn default() -> Self {
        Self{
            name : String::default("Machene ohne namen"),
            hitbox : Rect::default(),
            interaction_area:Rect::default(),
            state: State::Broken,
            sprite : MaschineSprite,
            trades: vec![],
            running_recources: Ressourcen,
          //  sender: NULL,
        }
    }
}

impl Maschine {

    pub fn new(namen: String, trades: Vec<Trade>) -> Self {

        let loadedSprite: MaschineSprite =  AssetService::get(name);

        Self {
            name: namen,
            hitbox: Default::default(),
            interaction_area: Default::default(),
            state: State::Broken,
            sprite: Default::default(),
            trades,
            running_recources: (),
         //   sender: ()
        }
    }


    pub fn no_energy(&mut self){
        self.state = State::Idel;
        //timer pausiren

    }
}

impl Area for Maschine
{
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