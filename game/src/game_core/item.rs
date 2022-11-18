use ggez::graphics::{Image, Rect};
use serde::{Deserialize, Serialize};
/// Defines an item in the inventory of the player
/// Contains the name of the item, information about the item and the image
#[derive(Clone, Eq, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Item {
    name: String,
    info_text: String,
    //image should be a texture, didnt work yet
    img: String,
}

impl Default for Item
{
    fn default() -> Self {
        Self
        {
            name: "Default Item".to_string(),
            info_text: "lock at this info".to_string(),
            img: "image".to_string()
        }
    }
}

