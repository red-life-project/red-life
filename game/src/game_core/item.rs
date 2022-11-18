use serde::{Deserialize, Serialize};
/// Defines an item in the inventory of the player
/// Contains the name of the item, information about the item and the image
#[derive(Clone, Eq, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Item {
    name: String,
    info_text: String,
    position: (usize, usize),
    //image should be a texture, didnt work yet
    img: String,
    amount: i16,
}
impl Item {
    pub fn change_amount(&mut self, amount: i16) {
        self.amount += amount;
    }
}