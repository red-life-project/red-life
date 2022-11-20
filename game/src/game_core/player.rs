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
    pub(crate) last_damage: u32,
}
impl Default for Player {
    fn default() -> Self {
        Self {
            inventory: vec![],
            position: (600, 500),
            resources: Resources {
                oxygen: u16::MAX,
                energy: u16::MAX,
                life: u16::MAX,
            },
            resources_change: Resources {
                oxygen: -1,
                energy: -1,
                life: 0,
            },
            last_damage: 0,
        }
    }
}

impl Player {
    /// Checks whether the player has taken damage in the past few seconds and if not so start the regeneration
    pub(crate) fn life_regeneration(&mut self) {
        if self.resources_change.life == 0 && self.last_damage >= 1000 {
            self.resources_change.life += 5;
            self.last_damage = 0;
        }
        if self.resources.life == u16::MAX && self.resources_change.life > 0 {
            self.resources_change.life = 0;
        } else {
            self.last_damage += 1;
        }
    }
}
