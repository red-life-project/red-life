use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

use std::sync::mpsc::Sender;

use crate::backend::constants::PLAYER_INTERACTION_RADIUS;
use crate::backend::gamestate::{GameCommand, GameState};
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
use crate::RLResult;
use ggez::graphics::{Color, Image, Rect};
use tracing::info;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum State {
    Broken,
    Idle,
    Running,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Machine {
    pub name: String,
    pub state: State,
    pub hit_box: Rect,
    interaction_area: Rect,
    trades: Vec<Trade>,
    last_trade: Trade,
    running_resources: Resources<i16>,
    time_remaining: i16,
    time_change: i16,
    #[serde(skip)]
    sprite: Option<MachineSprite>,
    #[serde(skip)]
    sender: Option<Sender<GameCommand>>,
}

impl Machine {
    pub(crate) fn is_interactable(&self, pos: (usize, usize)) -> bool {
        is_colliding(pos, &self.get_interaction_area())
    }
    pub fn new_by_const(
        gs: &GameState,
        sender: Sender<GameCommand>,
        (name, hit_box, trades, running_resources): (String, Rect, Vec<Trade>, Resources<i16>),
    ) -> RLResult<Self> {
        Machine::new(gs, name, hit_box, trades, running_resources, sender)
    }

    pub fn new(
        gs: &GameState,
        name: String,
        hit_box: Rect,
        trades: Vec<Trade>,
        running_resources: Resources<i16>,
        sender: Sender<GameCommand>,
    ) -> RLResult<Self> {
        info!("Creating new machine: name: {}", name);

        let sprite = MachineSprite::new(gs, name.as_str())?;
        Ok(Self {
            name,
            hit_box,
            interaction_area: Rect {
                x: hit_box.x - PLAYER_INTERACTION_RADIUS,
                y: hit_box.y - PLAYER_INTERACTION_RADIUS,
                w: hit_box.w + (PLAYER_INTERACTION_RADIUS * 2.),
                h: hit_box.h + (PLAYER_INTERACTION_RADIUS * 2.),
            },
            state: State::Broken,
            sprite: Some(sprite),
            trades,
            last_trade: Trade::default(),
            running_resources,

            time_remaining: 0,
            time_change: 0,
            sender: Some(sender),
        })
    }
    pub fn no_energy(&mut self) {
        self.state = State::Idle;
        //TODO: timer pausieren
    }
    fn get_trade(&self) -> Trade {
        // returns the first possible trade
        if let Some(t) = self.trades.iter().find(|t| t.initial_state == self.state) {
            return t.clone();
        }
        Trade::default()
    }

    fn check_change(&self, before: &State, after: &State) {
        match (before, after) {
            (Broken, Idle) | (Idle, Broken) => {}
            (Broken | Idle, Running) => {
                let _e = self
                    .sender
                    .as_ref()
                    .unwrap()
                    .send(GameCommand::ResourceChange(self.running_resources));
            }
            (Running, Broken | Idle) => {
                let _e = self
                    .sender
                    .as_ref()
                    .unwrap()
                    .send(GameCommand::ResourceChange(
                        // 0-n = n*-1  = n.invert()                            // TODO: add .invert() to Resources
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
    pub(crate) fn interact(
        &mut self,
        player: &mut Player,
        sender: &Sender<StackCommand>,
    ) -> Player {
        let trade = self.get_trade();
        let dif = trade
            .cost
            .iter()
            .map(|(item, demand)| (item, player.get_item_amount(item) - demand))
            .filter(|(_item, dif)| *dif < 0)
            .collect::<Vec<(&Item, i32)>>();
        if dif.iter().any(|(_, demand)| *demand < 0) {
            let mut missing_items = String::new();
            dif.iter()
                .map(|(item, amount)| format!("{amount} {}\n", item.name))
                .for_each(|x| missing_items.push_str(&x));

            let popup = Popup::new(
                RLColor::BLUE,
                format!("{}\n{missing_items}", TRADE_CONFLICT_POPUP[0]),
                5,
            );
            info!(
                "Popup for Trade conflict sent: Missing Items: {}",
                missing_items
            );
            sender.send(StackCommand::Popup(popup)).unwrap();
            return player.clone();
        }

        // all checks have been pased taking items
        info!("Executing trade:{} ", trade.name);
        self.last_trade = trade.clone();
        self.time_remaining = trade.time_ticks;
        if trade.time_ticks > 0 {
            self.time_change = 1;
        }
        trade
            .cost
            .iter()
            .for_each(|(item, demand)| player.add_item(item, -*demand));

        if self.state != trade.resulting_state {
            // if the state changed
            self.check_change(&self.state, &trade.resulting_state);
            self.state = trade.resulting_state;
        }

        player.clone()
    }

    pub(crate) fn get_collision_area(&self) -> Rect {
        self.hit_box
    }

    fn get_interaction_area(&self) -> Rect {
        self.interaction_area
    }

    pub(crate) fn get_graphic(&self) -> Image {
        match self.state {
            Broken => self.sprite.as_ref().unwrap().broken.clone(),
            Idle => self.sprite.as_ref().unwrap().idle.clone(),
            Running => self.sprite.as_ref().unwrap().running.clone(),
        }
    }
    /// Loads the Machine Sprites. Has to be called before drawing.
    pub(crate) fn load_sprites(&mut self, images: &[Image]) {
        self.sprite = Some(images.into());
    }

    pub(crate) fn is_non_broken_machine(&self) -> bool {
        self.state != Broken
    }

    pub(crate) fn get_name(&self) -> String {
        self.name.clone()
    }

    pub(crate) fn tick(&mut self, delta_tics: i16) {
        self.time_remaining -= self.time_change * delta_tics;
        if self.time_remaining < 0 {
            //timer run out
            self.time_change = 0;
            self.time_remaining = 0;

            if self.state != self.last_trade.initial_state {
                self.check_change(&self.state, &self.last_trade.initial_state);
                self.state = self.last_trade.initial_state.clone();
            }
        }
    }
    pub(crate) fn get_state(&self) -> State {
        self.state.clone()
    }

    pub(crate) fn get_time_percentage(&self) -> f32 {
        let x = if self.last_trade.time_ticks == 0 {
            -1.0
        } else {
            f32::from(self.time_remaining) / f32::from(self.last_trade.time_ticks)
        };
        x
    }
}
