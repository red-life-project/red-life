use crate::backend::constants::DESIRED_FPS;
use crate::backend::rlcolor::RLColor;
use crate::backend::screen::{Popup, StackCommand};
use crate::game_core::item::Item;
use crate::game_core::resources::Resources;
use crate::languages::german::GAME_INFO;
use crate::languages::german::{BENZIN, GEDRUCKTESTEIL, SUPER_GLUE};
use serde::{Deserialize, Serialize};
use std::sync::mpsc::Sender;
use tracing::info;

/// The current game player, containing its inventory and the current position, air and energy,
/// along with their change rate
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Player {
    /// The current items of the player.
    pub(crate) inventory: Vec<(Item, i32)>,
    pub(crate) position: (usize, usize),
    pub(crate) resources: Resources<u16>,
    pub(crate) resources_change: Resources<i16>,
    /// The current milestone the player has reached.
    pub milestone: usize,
    pub(crate) last_damage: u32,
    pub(crate) match_milestone: i8,
    pub(crate) time: u32,
}
impl Default for Player {
    fn default() -> Self {
        info!("Default Player created");
        Self {
            inventory: vec![
                (Item::new(SUPER_GLUE), 0),
                (Item::new(BENZIN), 3),
                (Item::new(GEDRUCKTESTEIL), 1),
            ],
            position: (600, 500),
            resources: Resources {
                oxygen: u16::MAX,
                energy: u16::MAX,
                life: u16::MAX,
            },
            resources_change: Resources {
                oxygen: 0,
                // In release Version this Value shouldplayer.inventory.0  be 0
                energy: -1,
                life: 0,
            },
            milestone: 0,
            last_damage: 0,
            match_milestone: 0,
            time: 0,
        }
    }
}

impl Player {
    /// Checks whether the player has taken damage in the past few seconds and if not so start the regeneration
    pub(crate) fn life_regeneration(&mut self, sender: &Sender<StackCommand>) {
        match (
            self.resources_change.life,
            self.last_damage,
            self.resources.life,
        ) {
            // If Player has full life and is healing, stop healing, reset last damage
            (change_life, _, u16::MAX) if change_life >= 0 => {
                if self.resources_change.life > 0 {
                    info!("Player has full life, stopping healing");
                }
                self.resources_change.life = 0;
                self.last_damage = 0;
            }
            // If player is healing reset last damage point
            (change_life, last_damage, _) if change_life > 0 && last_damage > 0 => {
                self.last_damage = 0;
            }
            // If player does not take damage and 5 seconds have passed, start healing
            (0, last_damage, _) if last_damage >= 8 * DESIRED_FPS => {
                self.resources_change.life += 5;
                self.last_damage = 0;
                let popup = Popup::new(RLColor::GREEN, GAME_INFO[0].to_string(), 5);
                info!("Player startet healing");
                sender.send(StackCommand::Popup(popup)).unwrap();
            }
            // If player takes damage, increase last damage point
            (change_life, _, _) if change_life < 0 => self.last_damage = 0,
            // Else, increase last damage point
            _ => self.last_damage += 1,
        }
    }
    pub fn add_item(&mut self, item: &Item, n: i32) {
        self.inventory.iter_mut().for_each(|(i, amount)| {
            if i.name == item.name {
                *amount += n;
            }
        });
    }
    pub fn get_item_amount(&self, item: &Item) -> i32 {
        let mut ret: i32 = -100;
        self.inventory.iter().for_each(|(i, amount)| {
            if i.name == item.name {
                ret = *amount;
            }
        });
        ret
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::backend::gamestate::GameState;
    use crate::backend::screen::Screen;
    use std::sync::mpsc::{channel, Receiver};

    fn setup_gamestate() -> (GameState, Receiver<StackCommand>) {
        let mut gamestate = GameState::default();
        let channel = channel();
        gamestate.set_sender(channel.0);
        (gamestate, channel.1)
    }
    #[test]
    fn test_case_one_life_regeneration() {
        let (gamestate, _) = setup_gamestate();
        let mut player = Player::default();
        player.resources.life = u16::MAX;
        player.resources_change.life = 5;
        player.last_damage = 1000;
        player.life_regeneration(&gamestate.screen_sender.as_ref().unwrap().clone());
        assert_eq!(player.resources_change.life, 0);
        assert_eq!(player.last_damage, 0);
    }

    #[test]
    fn test_case_two_life_regeneration() {
        let (gamestate, _) = setup_gamestate();
        let mut player = Player::default();
        player.resources.life = 1000;
        player.resources_change.life = 5;
        player.last_damage = 1000;
        player.life_regeneration(&gamestate.screen_sender.as_ref().unwrap().clone());
        assert_eq!(player.last_damage, 0);
    }

    #[test]
    fn test_case_three_life_regeneration() {
        let (gamestate, _receiver) = setup_gamestate();
        let mut player = Player::default();
        player.resources.life = 1000;
        player.resources_change.life = 0;
        player.last_damage = 900;
        player.life_regeneration(&gamestate.screen_sender.as_ref().unwrap().clone());
        assert_eq!(player.resources_change.life, 5);
        assert_eq!(player.last_damage, 0);
    }

    #[test]
    fn test_case_four_life_regeneration() {
        let (gamestate, _) = setup_gamestate();
        let mut player = Player::default();
        player.resources.life = 20000;
        player.last_damage = 400;
        player.resources_change.life = 0;
        player.life_regeneration(&gamestate.screen_sender.as_ref().unwrap().clone());
        assert_eq!(player.resources_change.life, 0);
        assert_eq!(player.last_damage, 401);
    }

    #[test]
    fn test_case_five_life_regeneration() {
        let (mut gamestate, _) = setup_gamestate();
        let channel = channel();
        gamestate.set_sender(channel.0);
        let mut player = Player {
            last_damage: 3,
            resources_change: Resources {
                life: -1,
                ..Default::default()
            },
            ..Player::default()
        };
        player.life_regeneration(&gamestate.screen_sender.as_ref().unwrap().clone());
        assert_eq!(player.resources_change.life, -1);
        assert_eq!(player.last_damage, 0);
    }
}
