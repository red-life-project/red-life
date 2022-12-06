#![warn(clippy::pedantic)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_possible_truncation)]
mod backend;
mod game_core;
mod languages;
mod machines;
mod main_menu;

use crate::backend::constants::SCREEN_RESOLUTION;
use crate::backend::{error, screen::Screenstack};
use chrono::Local;

use ggez::{event, Context};
use std::fs::File;
use std::sync::Mutex;
use tracing::{info, Level};

/// Our own Result Type for custom Error handling.
pub type RLResult<T = ()> = Result<T, error::RLError>;

pub fn main() -> RLResult {
    let cb = ggez::ContextBuilder::new("red-life", "red-life-project")
        .resources_dir_name("assets")
        .window_setup(
            ggez::conf::WindowSetup::default()
                .icon("/icon.png")
                .title("Red Life")
                .vsync(true),
        );
    // Start logging
    // Check if log folder exists
    if !std::path::Path::new("logs").exists() {
        std::fs::create_dir("logs").expect("Could not create log folder");
    }
    let filename = format!("logs/RL-{}.log", Local::now().format("%Y-%m-%d_%H-%M-%S"));
    let log_file = File::create(filename).unwrap();
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .with_writer(Mutex::new(log_file))
        .with_ansi(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    // End logging
    info!("Starting Red Life");
    let (mut ctx, event_loop) = cb.build()?;
    info!("New Event Loop created");
    window_setup(&mut ctx)?;
    let screen_stack = Screenstack::default();
    event::run(ctx, event_loop, screen_stack);
}

fn window_setup(ctx: &mut Context) -> RLResult {
    ctx.gfx.set_resizable(true)?;
    ctx.gfx
        .set_drawable_size(SCREEN_RESOLUTION.0, SCREEN_RESOLUTION.1)?;
    // If we're in a release build set fullscreen to true
    #[cfg(not(debug_assertions))]
    ctx.gfx.set_fullscreen(FullscreenType::True)?;
    Ok(())
}
