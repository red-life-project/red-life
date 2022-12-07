use serde::{Deserialize, Serialize};
use tracing::info;

/// Defines an item in the inventory of the player
/// Contains the name of the item, information about the item and the image
#[derive(Clone, Default, Eq, Debug, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    info_text: String,
    pub img: String,
}
impl Item {
    /// Create new `Item`
    /// # Arguments
    /// * `item` - 3 long String-Array, which defines the name, info text and image path for the created item
    pub(crate) fn new(item: [&str; 3]) -> Self {
        info!(
            "New Item created: name: {}, info_text: {}, img path: {}",
            item[0], item[1], item[2]
        );
        Self {
            name: item[0].to_string(),
            info_text: item[1].to_string(),
            img: item[2].to_string(),
        }
    }
}
