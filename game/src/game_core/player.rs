use crate::backend::constants::DESIRED_FPS;
use crate::backend::rlcolor::RLColor;
use crate::backend::screen::{Popup, StackCommand};
use crate::game_core::item::Item;
use crate::game_core::resources::Resources;
use crate::languages::*;
use crate::RLResult;
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
    /// The current air, energy and life of the player.
    pub(crate) resources: Resources<u16>,
    /// The current change rate of the air, energy and life of the player.
    pub(crate) resources_change: Resources<i16>,
    /// The current milestone the player has reached.
    pub milestone: usize,
    /// helper variable to check if the player lost life in the last tick
    pub(crate) last_damage: u32,
    /// contains the current ingame time
    pub(crate) time: u32,
}

impl Player {
    pub fn new(lng: Lang) -> Self {
        info!("Default Player created");
        Self {
            inventory: vec![
                (Item::new(*super_glue(lng)), 0),
                (Item::new(*petrol(lng)), 3),
                (Item::new(*printed_part(lng)), 1),
            ],
            position: (600, 500),
            resources: Resources {
                oxygen: u16::MAX,
                energy: u16::MAX,
                life: u16::MAX,
            },
            resources_change: Resources {
                oxygen: -5,
                energy: -10,
                life: 0,
            },
            milestone: 0,
            last_damage: 0,
            time: 0,
        }
    }
    /// Checks whether the player has taken damage in the past few seconds and if not so start the regeneration
    /// # Arguments
    /// * `sender` - The sender of the screen, needed to send a `Popup` to the screen.
    /// # Returns
    /// * `RLResult` - validates if life regeneration was started correctly
    pub(crate) fn life_regeneration(
        &mut self,
        sender: &Sender<StackCommand>,
        lng: Lang,
    ) -> RLResult {
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
                let popup = Popup::new(RLColor::GREEN, game_info(lng)[0].to_string(), 5);
                info!("Player started healing");
                sender.send(StackCommand::Popup(popup))?;
            }
            // If player takes damage, increase last damage point
            (change_life, _, _) if change_life < 0 => self.last_damage = 0,
            // Else, increase last damage point
            _ => self.last_damage += 1,
        }
        Ok(())
    }
    /// changes the amount of an specific item in the inventory by a given number
    /// # Arguments
    /// * `item` - The item to change the amount of
    /// * `amount_change` - The amount to change the item by
    pub fn add_item(&mut self, item: &Item, amount_change: i32) {
        self.inventory.iter_mut().for_each(|(i, amount)| {
            if i.name == item.name {
                *amount += amount_change;
            }
        });
    }
    /// returns the amount of an specific item in the inventory
    /// # Arguments
    /// * `item` - The item to get the amount of
    /// # Returns
    /// `ret` - The amount of the chosen item in the inventory or if the item is not in the
    /// inventory -100
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

/// Returns a Inventory with set sizes for all items
/// # Arguments
/// * `super_glue` - The amount of super glue
/// * `benzin` - The amount of benzin
/// * `gedrucktesteil` - The amount of the printed part
pub fn gen_inventory(
    super_glue_amount: i32,
    benzin_amount: i32,
    printed_parts_amount: i32,
    lng: Lang,
) -> Vec<(Item, i32)> {
    vec![
        (Item::new(*super_glue(lng)), super_glue_amount),
        (Item::new(*petrol(lng)), benzin_amount),
        (Item::new(*printed_part(lng)), printed_parts_amount),
    ]
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
        let (mut gamestate, _) = setup_gamestate();
        let mut player = Player::default();
        player.resources.life = u16::MAX;
        player.resources_change.life = 5;
        player.last_damage = 1000;
        player
            .life_regeneration(&gamestate.get_screen_sender().unwrap().clone())
            .unwrap();
        assert_eq!(player.resources_change.life, 0);
        assert_eq!(player.last_damage, 0);
    }

    #[test]
    fn test_case_two_life_regeneration() {
        let (mut gamestate, _) = setup_gamestate();
        let mut player = Player::default();
        player.resources.life = 1000;
        player.resources_change.life = 5;
        player.last_damage = 1000;
        player
            .life_regeneration(&gamestate.get_screen_sender().unwrap().clone())
            .unwrap();
        assert_eq!(player.last_damage, 0);
    }

    #[test]
    fn test_case_three_life_regeneration() {
        let (mut gamestate, _receiver) = setup_gamestate();
        let mut player = Player::default();
        player.resources.life = 1000;
        player.resources_change.life = 0;
        player.last_damage = 900;
        player
            .life_regeneration(&gamestate.get_screen_sender().unwrap().clone())
            .unwrap();
        assert_eq!(player.resources_change.life, 5);
        assert_eq!(player.last_damage, 0);
    }

    #[test]
    fn test_case_four_life_regeneration() {
        let (mut gamestate, _) = setup_gamestate();
        let mut player = Player::default();
        player.resources.life = 20000;
        player.last_damage = 400;
        player.resources_change.life = 0;
        player
            .life_regeneration(&gamestate.get_screen_sender().unwrap().clone())
            .unwrap();
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
        player
            .life_regeneration(&gamestate.get_screen_sender().unwrap().clone(), Lang::De)
            .unwrap();
        assert_eq!(player.resources_change.life, -1);
        assert_eq!(player.last_damage, 0);
    }
}
