use crate::game_core::item::Item;
use crate::game_core::resources::Resources;
use serde::{Deserialize, Serialize};

/// The current game player, containing its inventory and the current position, air and energy,
/// along with their change rate
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Player {
    /// The current items of the player.
    inventory: Vec<Item>,
    pub(crate) position: (usize, usize),
    pub(crate) resources: Resources<u16>,
    pub(crate) resources_change: Resources<i16>,
}
impl Default for Player {
    fn default() -> Self {
        Self {
            inventory: vec![],
            position: (0, 0),
            resources: Resources {
                oxygen: u16::MAX,
                energy: u16::MAX,
                life: u16::MAX,
            },
            resources_change: Resources {
                oxygen: -100,
                energy: -1,
                life: -100,
            },
        }
    }
}
