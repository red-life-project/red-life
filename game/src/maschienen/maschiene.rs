use std::default::default;
use ggez::graphics::Rect;
use crate::maschienen::maschiene_sprite::MaschineSprite;

pub enum State {
    Broken,
    Idel,
    Running
}

pub struct Maschine {
    name: String,
    hitbox: Rect,
    interaction_area: Rect,
    state: State,
    sprite: MaschineSprite,
    trades: Vec<Trade>,

}
impl Default for Maschine
{
    fn default() -> Self {
        Self{
            name : String::default("Machiene ohne namen"),
            hitbox : Rect::default(),
            interaction_area:Rect::default(),
            state: State::Broken,
            sprite : MaschineSprite,
            trades: vec![],
        }
    }
}

impl Maschine {

    pub fn new(name: String, trades: Vec<Trade>) -> Self {

        let loadedSprite: MaschineSprite =  AssetService::get(name);
        let default = default():Maschine;
        Self { name, hitbox:default.hitbox, interaction_area, state, sprite: loadedSprite, trades }
    }




}