use crate::backend::gamestate::{GameCommand, GameState};
use crate::backend::screen::{Screen, ScreenCommand, StackCommand};
use crate::backend::utils::{get_draw_params, get_scale};
use crate::languages::{
    additional_info_string, air_and_energy_string, air_string, button_info, death_reason_string,
    energy_string, intro_text, tutorial_text, winning_text, Lang,
};
use crate::main_menu::main_menu::MainMenu;
use crate::{draw, RLResult};
use ggez::glam::Vec2;
use ggez::winit::event::VirtualKeyCode;
use ggez::{graphics, Context};
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

impl DeathReason {
    fn t(self, lng: Lang) -> &'static str {
        match self {
            DeathReason::Oxygen => air_string(lng),
            DeathReason::Energy => energy_string(lng),
            DeathReason::Both => air_and_energy_string(lng),
        }
    }
}
/// Defines the type of Screen which is Info screen currently showing
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ScreenType {
    Death,
    Intro,
    Winning,
}

/// Create `DeathScreen`, `IntroScreen` or `WinningScreen`. `DeathScreen` needs the reason of death from `DeathReason` enum.
#[derive(Debug)]
pub struct InfoScreen {
    background: &'static str,
    main_message: graphics::Text,
    additional_text: graphics::Text,
    sender: Sender<StackCommand>,
    screen_type: ScreenType,
    background_image: Option<graphics::Image>,
    lng: Lang,
}

impl InfoScreen {
    /// Creates a new `DeathScreen` using `InfoScreen` with a `Death reason`
    /// # Arguments
    /// * `death_reason` - The reason for the death of the player
    /// * `sender` - The sender to send the command to the `ScreenStack`
    pub fn new_death_screen(
        death_reason: DeathReason,
        sender: Sender<StackCommand>,
        lng: Lang,
    ) -> Self {
        info!(
            "The player died due to a lack of : {:?}",
            death_reason.t(lng)
        );

        let mut main_message = graphics::Text::new(format!(
            "{} {}",
            death_reason_string(lng),
            death_reason.t(lng)
        ));
        main_message.set_scale(70.);
        let mut additional_text = graphics::Text::new(additional_info_string(lng));
        additional_text.set_scale(70.);
        let background = "deathscreen";
        let screentype = ScreenType::Death;
        Self {
            background,
            main_message,
            additional_text,
            sender,
            screen_type: screentype,
            background_image: None,
            lng,
        }
    }
    /// Creates a new `IntroScreen` using `InfoScreen`
    /// # Arguments
    /// * `sender` - The sender to send the command to the `ScreenStack`
    pub fn new_intro_screen(sender: Sender<StackCommand>, lng: Lang) -> Self {
        let mut main_message =
            graphics::Text::new(format!("{} \n{}", intro_text(lng), tutorial_text(lng)));
        main_message.set_scale(50.);
        let mut additional_text = graphics::Text::new(button_info(lng));
        additional_text.set_scale(50.);
        let background = "Introscreen";
        let screen_type = ScreenType::Intro;
        Self {
            background,
            main_message,
            additional_text,
            sender,
            screen_type,
            background_image: None,
            lng,
        }
    }
    /// Creates a new Winning using `InfoScreen`
    /// # Arguments
    /// * `sender` - The sender to send the command to the `ScreenStack`
    pub fn new_winning_screen(sender: Sender<StackCommand>, lng: Lang) -> Self {
        let mut main_message = graphics::Text::new(winning_text(lng));
        main_message.set_scale(70.);
        let mut additional_text = graphics::Text::new(additional_info_string(lng));
        additional_text.set_scale(70.);
        let background = "Winningscreen";
        let screen_type = ScreenType::Winning;
        Self {
            background,
            main_message,
            additional_text,
            sender,
            screen_type,
            background_image: None,
            lng,
        }
    }
}
/// Implement the `Screen` trait for `InfoScreen`
impl Screen for InfoScreen {
    /// Updates the screen every tick, checks if esc or space is pressed
    /// # Arguments
    /// * `ctx` - The ggez context
    /// # Returns
    /// `RLResult` - Returns an `RLResult`.
    fn update(&mut self, ctx: &mut Context) -> RLResult {
        let lng = self.lng;
        if self.background_image.is_none() {
            self.background_image = Some(graphics::Image::from_bytes(
                ctx,
                fs::read(format!("assets/{}.png", self.background).as_str())?.as_slice(),
            )?);
        }
        let keys = ctx.keyboard.pressed_keys();
        // Here we only use the first pressed key, but in the info screen this is fine
        match (self.screen_type, keys.iter().next()) {
            (ScreenType::Intro, Some(&VirtualKeyCode::Space)) => {
                self.sender.send(StackCommand::Screen(ScreenCommand::Pop))?;
                self.sender
                    .send(StackCommand::Screen(ScreenCommand::Push(Box::new({
                        let mut game_state = GameState::new(ctx, lng)?;
                        game_state.init(ctx)?;
                        game_state.create_machine();
                        game_state
                            .sender
                            .as_mut()
                            .unwrap()
                            .send(GameCommand::Milestone)?;
                        game_state
                    }))))?;
            }
            (ScreenType::Death | ScreenType::Winning, Some(&VirtualKeyCode::Escape)) => {
                if self.screen_type == ScreenType::Winning {
                    GameState::delete_saves()?;
                }
                self.sender.send(StackCommand::Screen(ScreenCommand::Pop))?;
                self.sender
                    .send(StackCommand::Screen(ScreenCommand::Push(Box::new(
                        MainMenu::new(self.sender.clone(), lng),
                    ))))?;
            }
            _ => {}
        }
        Ok(())
    }
    /// Draws the info screen with the given background and two texts
    /// # Arguments
    /// * `ctx` - The ggez context
    /// # Returns
    /// `RLResult` - Returns an `RLResult`.
    fn draw(&self, ctx: &mut Context) -> RLResult {
        let scale = get_scale(ctx);
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::RED);

        if let Some(background) = &self.background_image {
            canvas.draw(background, graphics::DrawParam::default().scale(scale));
        }
        if self.screen_type == ScreenType::Intro {
            draw!(canvas, &self.main_message, Vec2::new(300., 300.), scale);
        } else {
            draw!(canvas, &self.main_message, Vec2::new(220., 500.), scale);
        }

        draw!(canvas, &self.additional_text, Vec2::new(646., 740.), scale);

        canvas.finish(ctx)?;

        Ok(())
    }

    fn set_sender(&mut self, sender: Sender<StackCommand>) {
        self.sender = sender;
    }

    fn lang(&self) -> Lang {
        self.lng
    }
}
