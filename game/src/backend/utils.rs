//! miscellaneous utilities used by the backend
use crate::backend::constants::{PLAYER_ICON_SIZE, SCREEN_RESOLUTION};
use crate::game_core::item::Item;
use crate::languages::german::{BENZIN, GEDRUCKTESTEIL, SUPER_GLUE};
use ggez::glam::Vec2;
use ggez::graphics::Rect;
use ggez::Context;

/// This returns the scale so we can have resolution-agnostic scaling
/// Use it in your drawing calls like this:
/// ```rust
/// let scale = get_scale(ctx);
/// graphics::draw(ctx, &self.img, graphics::DrawParam::default().scale(scale))?;
/// ```
#[inline(always)]
pub fn get_scale(ctx: &Context) -> Vec2 {
    let (width, height) = ctx.gfx.drawable_size();
    Vec2::new(width / SCREEN_RESOLUTION.0, height / SCREEN_RESOLUTION.1)
}

/// Returns if the player would collide with a machine if they moved in the given direction
#[inline(always)]
pub fn is_colliding(player_pos: (usize, usize), area: &Rect) -> bool {
    area.x < player_pos.0 as f32 + PLAYER_ICON_SIZE.0 as f32
        && area.x + area.w > player_pos.0 as f32
        && area.y < player_pos.1 as f32 + PLAYER_ICON_SIZE.1 as f32
        && area.y + area.h > player_pos.1 as f32
}
///Returns a Inventory with set sizen for all items
#[inline(always)]
pub fn gen_inventory(super_glue: i32, benzin: i32, gedrucktesteil: i32) -> Vec<(Item, i32)> {
    vec![
        (Item::new(SUPER_GLUE), super_glue),
        (Item::new(BENZIN), benzin),
        (Item::new(GEDRUCKTESTEIL), gedrucktesteil),
    ]
}

pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


/// This macro is used for simplifying drawing with scaling.
/// It takes a canvas, a `Drawable`, an (optional) position(as `Vec2` for example) and a scale as `Vec2`.
#[macro_export]
macro_rules! draw {
    ($canvas: expr, $asset: expr, $position: expr, $scale: expr) => {
        $canvas.draw(
            $asset,
            ggez::graphics::DrawParam::default()
                .dest($position * $scale)
                .scale($scale),
        );
    };
    ($canvas: expr, $drawable: expr, $scale: expr) => {
        $canvas.draw(
            $drawable,
            ggez::graphics::DrawParam::default().scale($scale),
        );
    };
}
