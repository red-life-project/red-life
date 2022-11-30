use crate::backend::gamestate::GameState;
use crate::backend::screen::{Screen, StackCommand};
use crate::backend::utils::get_scale;
use crate::game_core::deathscreen::{DeathReason, DeathScreen};
use crate::languages::german::{ADDITIONAL_INFO_STRING, DEATH_REASON_STRING};
use crate::main_menu::mainmenu::MainMenu;
use crate::{draw, RLResult};
use ggez::glam::Vec2;
use ggez::winit::event::VirtualKeyCode;
use ggez::{graphics, Context};
use std::sync::mpsc::Sender;
use tracing::info;

#[derive(Debug)]
pub struct InfoScreen {
    background: String,
    main_message: graphics::Text,
    additional_text: graphics::Text,
    sender: Sender<StackCommand>,
}

impl InfoScreen {
    /// Creates a new DeathScreen using InfoScreen with a Deathreason
    /// # Arguments
    /// * `death_reason` - The reason for the death of the player
    /// * `sender` - The sender to send the command to the `ScreenStack`
    /// * `background` - The assetname of the background
    pub fn new_deathscreen(
        death_reason: DeathReason,
        sender: Sender<StackCommand>,
        background: String,
    ) -> Self {
        info!("The player died due to a lack of : {:?}", death_reason);

        let mut main_message =
            graphics::Text::new(format!("{} {death_reason}", DEATH_REASON_STRING));
        main_message.set_scale(70.);
        let mut additional_text = graphics::Text::new(ADDITIONAL_INFO_STRING);
        additional_text.set_scale(70.);

        Self {
            background,
            main_message,
            additional_text,
            sender,
        }
    }
    /// Creates a new IntroScreen using InfoScreen
    /// # Arguments
    /// * `death_reason` - The reason for the death of the player
    /// * `sender` - The sender to send the command to the `ScreenStack`
    /// * `background` - The assetname of the background
    pub fn new_introscreen(sender: Sender<StackCommand>, background: String) -> Self {
        let mut main_message = graphics::Text::new("Test");
        main_message.set_scale(70.);
        let mut additional_text = graphics::Text::new(ADDITIONAL_INFO_STRING);
        additional_text.set_scale(70.);

        Self {
            background,
            main_message,
            additional_text,
            sender,
        }
    }
}

impl Screen for InfoScreen {
    fn update(&mut self, ctx: &mut Context) -> RLResult {
        let keys = ctx.keyboard.pressed_keys();
        if let Some(key) = keys.iter().next() {
            info!("The player wants to got to the next screen with: {:?}", key);
            if key == &VirtualKeyCode::Escape {
                self.sender.send(StackCommand::Push(Box::new(MainMenu::new(
                    self.sender.clone(),
                ))))?;
            };
            if key == &VirtualKeyCode::Space {
                self.sender.send(StackCommand::Pop)?;
                self.sender.send(StackCommand::Push(Box::new({
                    let mut gamestate = GameState::load(false).unwrap_or_default();
                    gamestate.load_assets(ctx)?;
                    gamestate
                })))?;
            };
        }
        Ok(())
    }
    /// Draws the info screen with the given background and two texts
    fn draw(&self, ctx: &mut Context) -> RLResult {
        let scale = get_scale(ctx);
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::RED);
        let backgroundpath = format!("/{}.png", self.background);

        let background = graphics::Image::from_path(ctx, backgroundpath)?;

        canvas.draw(&background, graphics::DrawParam::default().scale(scale));

        draw!(canvas, &self.main_message, Vec2::new(372., 520.), scale);

        draw!(canvas, &self.additional_text, Vec2::new(646., 720.), scale);

        canvas.finish(ctx)?;

        Ok(())
    }

    fn set_sender(&mut self, sender: Sender<StackCommand>) {
        self.sender = sender;
    }
}
