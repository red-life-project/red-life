use crate::backend::constants::PLAYER_INTERACTION_RADIUS;
use crate::backend::gamestate::GameCommand;
use crate::backend::rlcolor::RLColor;
use crate::backend::screen::{Popup, StackCommand};
use crate::backend::utils::is_colliding;
use crate::game_core::item::Item;
use crate::game_core::player::Player;
use crate::game_core::resources::Resources;
use crate::languages::german::TRADE_CONFLICT_POPUP;
use crate::machines::machine::State::{Broken, Idle, Running};
use crate::machines::machine_sprite::MachineSprite;
use crate::machines::trade::Trade;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::sync::mpsc::Sender;

use crate::RLResult;
use ggez::graphics::{Color, Image, Rect};
use tracing::info;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum State {
    Broken,
    Idle,
    Running,
}
/// only used for logging purposes
impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Broken => write!(f, "Broken"),
            Idle => write!(f, "Idle"),
            Running => write!(f, "Running"),
        }
    }
}

impl From<State> for Color {
    fn from(value: State) -> Self {
        match value {
            Broken => RLColor::STATUS_RED,
            Idle => RLColor::STATUS_YELLOW,
            Running => RLColor::STATUS_GREEN,
        }
    }
}
/// The Machine Class handels any internal logik surrounding intractable objects
/// This includes objects that arnt classic Machines per se but since they do behave so similarity
/// we can reuse the same code for it
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Machine {
    /// The name of the machine is used as a reference on which assets to load
    pub name: String,
    /// Contains the current state
    pub state: State,
    /// The hitbox is the area the player is prevented from walking into
    pub hitbox: Rect,
    /// The interaction_area is the area the player has to be inside to interact with this Machine
    pub interaction_area: Rect,
    /// Contains a defined list of Trades, things the player can do with the Machine
    pub trades: Vec<Trade>,
    /// Contains the last trade with a timer, Is uses to get information about the trade after the timer runs out
    last_trade: Trade,
    /// Denotes what amount of Resources is consumed and or produced as long as the Machine is in state running
    running_resources: Resources<i16>,
    /// Contains the amount of tics until the timer rus out
    time_remaining: i16,
    /// Denotes weather the timer is running or not via 0 or 1 also used for calculations
    time_change: i16,
    #[serde(skip)]
    /// Contains all the Sprites for this one Machine
    sprite: Option<MachineSprite>,
    #[serde(skip)]
    /// Needed to send Messages to the `GameState` to make changes to the screen
    sender: Option<Sender<GameCommand>>,
    #[serde(skip)]
    /// Needed to send Messages to the `Screenstack` to make changes to the screen
    screen_sender: Option<Sender<StackCommand>>,
}

impl Machine {
    /// Creates a new Machine with all non Optional parameters
    /// # Arguments
    /// * `name` - Name of this Machine and asset group
    /// * `hitbox` - A rect containing position and size of the Machine
    /// * `trades` - A list of Trades
    /// * `running_resources` - Amount of recourse consumed and or produced while running
    /// # Returns
    /// + 'Machine'
    fn new(
        name: String,
        hitbox: Rect,
        trades: Vec<Trade>,
        running_resources: Resources<i16>,
    ) -> Self {
        info!("Creating new machine: name: {}", name);
        Self {
            name,
            hitbox,
            interaction_area: Rect {
                x: hitbox.x - PLAYER_INTERACTION_RADIUS,
                y: hitbox.y - PLAYER_INTERACTION_RADIUS,
                w: hitbox.w + (PLAYER_INTERACTION_RADIUS * 2.),
                h: hitbox.h + (PLAYER_INTERACTION_RADIUS * 2.),
            },
            state: Broken,
            sprite: None,
            trades,
            last_trade: Trade::default(),
            running_resources,
            time_remaining: 0,
            time_change: 0,
            sender: None,
            screen_sender: None,
        }
    }

    /// Alternative new constructor for the machine using one parameter Tupel
    /// # Arguments
    /// * `(name, hit_box, trades, running_resources)` - Tupel containing the same arguments as `new()`
    /// # Returns
    /// + 'Machine'
    pub(crate) fn new_by_const(
        (name, hit_box, trades, running_resources): (String, Rect, Vec<Trade>, Resources<i16>),
    ) -> Self {
        Machine::new(name, hit_box, trades, running_resources)
    }

    /// initialises the Maschine with the data that is not Serialize
    /// This funktion is required to be called before the firs draw call
    /// # Arguments
    /// * `images` - A Slice of Images containing the sprites for this Machine
    /// * `sender` - A sender of type `Sender<GameCommand>`
    /// * `screen_sender` - A sender of type `Sender<StackCommand>`
    pub(crate) fn init(
        &mut self,
        images: &[Image],
        sender: Sender<GameCommand>,
        screen_sender: Sender<StackCommand>,
    ) {
        self.sprite = Some(images.into());
        self.sender = Some(sender);
        self.screen_sender = Some(screen_sender);
        if self.name == "Loch" {
            // Constant (pos of hole)
            if self.hitbox.x == 780. {
                self.change_state_to(&Running);
            } else {
                self.change_state_to(&Idle);
            }
        }
    }

    /// Fetches the correct sprite depending on the current sate
    /// # Returns
    /// * `&Image` - a reference to the graphic
    pub(crate) fn get_graphic(&self) -> &Image {
        self.sprite.as_ref().unwrap().get(self.state.clone())
    }

    /// Calculates the Percentage of time remaining on the timer
    /// # Returns
    /// * `0...1` - '1' being timer just started equal to 100%
    pub(crate) fn get_time_percentage(&self) -> f32 {
        if self.last_trade.time_ticks == 0 {
            -1.0
        } else {
            f32::from(self.time_remaining) / f32::from(self.last_trade.time_ticks)
        }
    }

    /// Determines if the Player can interact with this Machine
    /// # Arguments
    /// * `pos` - a tuples of x and y containing the player position
    /// # Returns
    /// * `true` if the player collides with this Machine
    /// * `false` if the player does not collide with this Machine
    pub(crate) fn is_interactable(&self, pos: (usize, usize)) -> bool {
        is_colliding(pos, &self.interaction_area)
    }

    /// Handel's the interaction of the Maschine and the player
    /// # Arguments
    /// * `player` - of type `& Player` is a reference to the player
    pub(crate) fn interact(&mut self, player: &Player) -> RLResult {
        // Check if there is a possible trade
        let trade ;

        if let Some(t)= self.trades.iter().find(|t| t.initial_state == self.state){
            trade = t.clone();
        }else {
            return Ok(());
        }

        if trade.name == *"no_Trade" {
            return Ok(());
        }
        // Check if the player has energy (and its needed)
        if player.resources.energy == 0 && self.running_resources.energy < 0 && self.name != "Loch"
        {
            return Ok(());
        }
        // dif = the different between items the player has and the cost of the trade
        let dif = trade
            .cost
            .iter()
            .map(|(item, demand)| (item, player.get_item_amount(item) - demand))
            .filter(|(_item, dif)| *dif < 0)
            .collect::<Vec<(&Item, i32)>>();
        // If one item is not available in enough quantity inform the player and cancel the interaction
        if dif.iter().any(|(_, demand)| *demand < 0) {
            let mut missing_items = String::new();
            dif.iter()
                .map(|(item, amount)| format!("*{} {}\n", amount * -1, item.name))
                .for_each(|x| missing_items.push_str(&x));
            let popup = Popup::info(format!("{}\n{missing_items}", TRADE_CONFLICT_POPUP[0]));
            info!(
                "Popup for Trade conflict sent: Missing Items: {}",
                missing_items
            );
            self.screen_sender
                .as_ref()
                .unwrap()
                .send(StackCommand::Popup(popup))?;
            return Ok(());
        }

        // At this point all checks have passed and continue with executing the trade
        info!("Executing trade:{} ", trade.name);

        // Remove the cost of the trade from the players inventory by sending the demand to the AddItem GameCommand
        let items_cost = trade
            .cost
            .iter()
            .filter(|(_, demand)| *demand >= 0)
            .map(|(item, demand)| (item.clone(), -*demand))
            .collect::<Vec<(Item, i32)>>();
        self.sender
            .as_ref()
            .unwrap()
            .send(GameCommand::AddItems(items_cost))?;


        if trade.time_ticks == 0 {
            // this trade has no timer
            self.time_change = 0;
        } else {
            //this trade has a timer
            if self.time_remaining == 0 {
                //if no timer is running set timer up
                self.last_trade = trade.clone();
                self.time_remaining = trade.time_ticks;
            }
            //start the timer
            self.time_change = 1;
        }

        if trade.return_after_timer {
            self.change_state_to(&trade.resulting_state);
        }

        Ok(())
    }

    /// Handels the timer by being called every tick
    pub(crate) fn tick(&mut self) -> RLResult {
        self.time_remaining -= self.time_change;
        //if the timer has run out
        if self.time_remaining < 0 {
            //reset timer values and stop timer
            self.time_change = 0;
            self.time_remaining = 0;

            // handel edge case for wining the game
            if self.last_trade.name == "Notfall_signal_absetzen" {
                let _e = self.sender.as_ref().unwrap().send(GameCommand::Winning);
            }

            if self.last_trade.return_after_timer {
                self.change_state_to(&self.last_trade.initial_state.clone());
            } else {
                self.change_state_to(&self.last_trade.resulting_state.clone());
            }
            // After Trade ended, send the GameCommand AddItems to add the earning Items to the players inventory
            let trade = self.last_trade.clone();
            let items = trade
                .cost
                .iter()
                .filter(|(_, demand)| *demand < 0)
                .map(|(item, demand)| (item.clone(), -*demand))
                .collect::<Vec<(Item, i32)>>();
            self.sender
                .as_ref()
                .unwrap()
                .send(GameCommand::AddItems(items))?;
        }
        Ok(())
    }

    /// Used to change the State of the Machine gracefully
    /// # Arguments
    /// * `new_state` - the state that the machine should change to
    pub(crate) fn change_state_to(&mut self, new_state: &State) {
        if self.state != *new_state {
            self.invoke_state_change(&self.state, new_state);
            if self.state ==Running && self.time_change ==1 {
                self.time_change = 0;
            }
            self.state = new_state.clone();
        }
    }
    /// A helper funktion to disable every funktion in case there is no energy in the system
    pub(crate) fn no_energy(&mut self) {
        if self.running_resources.energy < 0 && self.name != "Loch" {
            // If there is no energy available but this machine needs some, stop this machine.
            if self.state == Running {
                self.change_state_to(&Idle);
            }
        }
    }

    /// Helper funktion that sends appropriate `GameCommand`s depending on change of the state
    /// Used to change the State of the Machine gracefully
    /// # Arguments
    /// * `before` - the current state of the system
    /// * `after` - the state that it will be in after the change is complete
    fn invoke_state_change(&self, before: &State, after: &State) {
        match (before, after) {
            (Broken, Idle) => {
                let _e = self.sender.as_ref().unwrap().send(GameCommand::Milestone);
            }
            (Idle, Broken) => {}
            (Broken | Idle, Running) => {
                let _e = self
                    .sender
                    .as_ref()
                    .unwrap()
                    .send(GameCommand::ResourceChange(self.running_resources));
                let _e = self.sender.as_ref().unwrap().send(GameCommand::Milestone);
            }
            (Running, Broken | Idle) => {
                let _e = self
                    .sender
                    .as_ref()
                    .unwrap()
                    .send(GameCommand::ResourceChange(
                        Resources {
                            oxygen: 0,
                            energy: 0,
                            life: 0,
                        } - self.running_resources,
                    ));
            }
            _ => {
                info!(
                    "unexpected case in Match. machine state changed from {} to {}",
                    before.clone(),
                    after.clone()
                );
            }
        }
    }
}
