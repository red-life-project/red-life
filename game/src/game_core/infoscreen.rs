use crate::backend::gamestate::GameState;
use crate::backend::screen::{Screen, StackCommand};
use crate::backend::utils::*;
use crate::languages::german::{
    ADDITIONAL_INFO_STRING, AIR_AND_ENERGY_STRING, AIR_STRING, BUTTON_INFO, DEATH_REASON_STRING,
    ENERGY_STRING, INTRO_TEXT, TUTORIAL_TEXT, WINNING_TEXT,
};
use crate::main_menu::mainmenu::MainMenu;
use crate::{draw, RLResult};
use ggez::glam::Vec2;
use ggez::winit::event::VirtualKeyCode;
use ggez::{graphics, Context};
use std::fmt::{Display, Formatter};
use std::fs;
use std::sync::mpsc::Sender;
use tracing::info;

/// Defines the reason for the death of the player and is used to display the reason on the screen
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DeathReason {
    Oxygen,
    Energy,
    Both,
}
impl Display for DeathReason {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DeathReason::Oxygen => write!(f, "{AIR_STRING}"),
            DeathReason::Energy => write!(f, "{ENERGY_STRING}"),
            DeathReason::Both => write!(f, "{AIR_AND_ENERGY_STRING}"),
        }
    }
}
/// Defines the type of Screen which is Infoscreen currently showing
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ScreenType {
    Death,
    Intro,
    Winning,
}
/// Create `DeathScreen`, `IntroScreen` or `WinningSreen`. DeathScreen needs the reason of death from `DeathReason` enum.
#[derive(Debug)]
pub struct InfoScreen {
    background: String,
    main_message: graphics::Text,
    additional_text: graphics::Text,
    sender: Sender<StackCommand>,
    screentype: ScreenType,
    background_image: Option<graphics::Image>,
}

impl InfoScreen {
    /// Creates a new DeathScreen using InfoScreen with a Deathreason
    /// # Arguments
    /// * `death_reason` - The reason for the death of the player
    /// * `sender` - The sender to send the command to the `ScreenStack`
    pub fn new_deathscreen(death_reason: DeathReason, sender: Sender<StackCommand>) -> Self {
        info!("The player died due to a lack of : {:?}", death_reason);

        let mut main_message = graphics::Text::new(format!("{DEATH_REASON_STRING} {death_reason}"));
        main_message.set_scale(70.);
        let mut additional_text = graphics::Text::new(ADDITIONAL_INFO_STRING);
        additional_text.set_scale(70.);
        let background = "deathscreen".to_string();
        let screentype = ScreenType::Death;
        Self {
            background,
            main_message,
            additional_text,
            sender,
            screentype,
            background_image: None,
        }
    }
    /// Creates a new IntroScreen using InfoScreen
    /// # Arguments
    /// * `sender` - The sender to send the command to the `ScreenStack`
    pub fn new_introscreen(sender: Sender<StackCommand>) -> Self {
        let mut main_message = graphics::Text::new(format!("{INTRO_TEXT} \n{TUTORIAL_TEXT}"));
        main_message.set_scale(50.);
        let mut additional_text = graphics::Text::new(BUTTON_INFO);
        additional_text.set_scale(50.);
        let background = "Introscreen".to_string();
        let screentype = ScreenType::Intro;
        Self {
            background,
            main_message,
            additional_text,
            sender,
            screentype,
            background_image: None,
        }
    }
    /// Creates a new Winning using InfoScreen
    /// # Arguments
    /// * `sender` - The sender to send the command to the `ScreenStack`
    pub fn new_winningscreen(sender: Sender<StackCommand>) -> Self {
        let mut main_message = graphics::Text::new(WINNING_TEXT);
        main_message.set_scale(70.);
        let mut additional_text = graphics::Text::new(ADDITIONAL_INFO_STRING);
        additional_text.set_scale(70.);
        let background = "Winningscreen".to_string();
        let screentype = ScreenType::Winning;
        Self {
            background,
            main_message,
            additional_text,
            sender,
            screentype,
            background_image: None,
        }
    }
}

impl Screen for InfoScreen {
    fn update(&mut self, ctx: &mut Context) -> RLResult {
        if self.background_image.is_none() {
            self.background_image = Some(graphics::Image::from_bytes(
                ctx,
                fs::read(format!("assets/{}.png", self.background).as_str())?.as_slice(),
            )?);
        }
        let keys = ctx.keyboard.pressed_keys();
        // Here we only use the first pressed key, but in the infoscreen this is fine
        match (self.screentype, keys.iter().next()) {
            (_, None) => Ok(()),
            (ScreenType::Intro, Some(key)) => match key {
                VirtualKeyCode::Space => {
                    self.sender.send(StackCommand::Pop)?;
                    Ok(self.sender.send(StackCommand::Push(Box::new({
                        let mut gamestate = GameState::new(ctx).unwrap_or_default();
                        gamestate.init(ctx)?;
                        gamestate.create_machine();
                        gamestate
                    })))?)
                }
                VirtualKeyCode::Escape => Ok(self.sender.send(StackCommand::Pop)?),
                _ => Ok(()),
            },
            (_, Some(VirtualKeyCode::Escape)) => Ok(self.sender.send(StackCommand::Push(
                Box::new(MainMenu::new(self.sender.clone())),
            ))?),
            _ => Ok(()),
        }
    }
    /// Draws the info screen with the given background and two texts
    fn draw(&self, ctx: &mut Context) -> RLResult {
        let scale = get_scale(ctx);
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::RED);

        if let Some(background) = &self.background_image {
            canvas.draw(background, graphics::DrawParam::default().scale(scale));
        }
        if self.screentype == ScreenType::Intro {
            draw!(canvas, &self.main_message, Vec2::new(300., 300.), scale);
        } else {
            draw!(canvas, &self.main_message, Vec2::new(372., 500.), scale);
        }

        draw!(canvas, &self.additional_text, Vec2::new(646., 740.), scale);

        canvas.finish(ctx)?;

        Ok(())
    }

    fn set_sender(&mut self, sender: Sender<StackCommand>) {
        self.sender = sender;
    }
}
