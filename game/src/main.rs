mod error;
mod gamestate;
mod mainmenu;
mod movement;
mod screen;
mod utils;

use crate::screen::Screenstack;
use ggez::conf::FullscreenType;
use ggez::{event, Context};

pub type RedResult<T = ()> = Result<T, error::RedError>;

pub fn main() -> RedResult {
    let cb = ggez::ContextBuilder::new("red-life", "red-life-project")
        .resources_dir_name("assets")
        .window_setup(
            ggez::conf::WindowSetup::default()
                .icon("/icon.png")
                .to_owned()
                .title("Red Life")
                .vsync(true),
        );
    let (mut ctx, event_loop) = cb.build()?;
    window_setup(&mut ctx)?;
    let screen_stack = Screenstack::default();
    event::run(ctx, event_loop, screen_stack);
}

fn window_setup(ctx: &mut Context) -> RedResult {
    ctx.gfx.set_resizable(true)?;
    ctx.gfx.set_drawable_size(1920., 1080.)?;
    // If we're in a release build set fullscreen to true
    #[cfg(not(debug_assertions))]
    ctx.gfx.set_fullscreen(FullscreenType::True)?;
    Ok(())
}
