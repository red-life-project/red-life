mod error;
mod gamestate;
mod mainmenu;
mod screen;

use crate::screen::Screenstack;
use ggez::{
    event,
};

type RedResult<T = ()> = Result<T, error::RedError>;

pub fn main() {
    let cb = ggez::ContextBuilder::new("red-life", "red-life-project").window_setup(
        ggez::conf::WindowSetup::default()
            .title("Red Life")
            .vsync(true),
    );
    let (ctx, event_loop) = cb.build().unwrap();
    let screen_stack = Screenstack::default();
    event::run(ctx, event_loop, screen_stack);
}
