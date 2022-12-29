//     Red Life, a small game about surviving on mars
//     Copyright (C) 2022  Red Life Team
//
//     This program is free software: you can redistribute it and/or modify
//     it under the terms of the GNU General Public License as published by
//     the Free Software Foundation, either version 3 of the License, or
//     (at your option) any later version.
//
//     This program is distributed in the hope that it will be useful,
//     but WITHOUT ANY WARRANTY; without even the implied warranty of
//     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//     GNU General Public License for more details.
//
//     You should have received a copy of the GNU General Public License
//     along with this program.  If not, see <https://www.gnu.org/licenses/>.
//! Starts the game and handles window configuration
#![warn(clippy::pedantic)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_possible_truncation)]
mod backend;
mod game_core;
mod languages;
mod machines;
mod main_menu;

use crate::backend::constants::SCREEN_RESOLUTION;
use crate::backend::{error, screen::ScreenStack};
use chrono::Local;

use crate::languages::Lang;
#[cfg_attr(debug_assertions, allow(unused_imports))]
use ggez::conf::FullscreenType;
use ggez::{event, Context};
use std::fs::File;
use std::sync::Mutex;
use tracing::{info, Level};

/// Our own Result Type for custom Error handling.
pub type RLResult<T = ()> = Result<T, error::RLError>;
/// The main function, which is the entry point of our program
/// builds the game and sets window configuration, icon and title
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
        std::fs::create_dir("logs")?;
    }
    let filename = format!("logs/RL-{}.log", Local::now().format("%Y-%m-%d_%H-%M-%S"));
    let log_file = File::create(filename)?;
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
    let lng = Lang::En;
    let screen_stack = ScreenStack::new_with_lang(lng);
    event::run(ctx, event_loop, screen_stack);
}
/// Sets the window size to resizeable in debug mode and fullscreen mode for release mode
fn window_setup(ctx: &mut Context) -> RLResult {
    ctx.gfx.set_resizable(true)?;
    ctx.gfx
        .set_drawable_size(SCREEN_RESOLUTION.0, SCREEN_RESOLUTION.1)?;
    // If we're in a release build set fullscreen to true
    #[cfg(not(debug_assertions))]
    ctx.gfx.set_fullscreen(FullscreenType::Desktop)?;
    Ok(())
}
