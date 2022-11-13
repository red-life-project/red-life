use ggez::graphics::{Image, Rect};

pub struct Item {
    name: String,
    img:Image,
    hitbox:Rect,
}
impl Default for Item
{
    fn default() -> Self {
        Self
        {
            name:String::default("item"),
            img: Image(),
            hitbox: Rect::default()
        }
    }
}
impl Item{


}