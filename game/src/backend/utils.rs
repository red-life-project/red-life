//! miscellaneous utilities used by the backend
use ggez::glam::Vec2;
use ggez::graphics::Rect;
use ggez::Context;
use std::cmp::{max, min};
/// This returns the scale so we can have resolution-agnostic scaling
/// Use it in your drawing calls like this:
/// ```rust
/// let scale = get_scale(ctx);
/// graphics::draw(ctx, &self.img, graphics::DrawParam::default().scale(scale))?;
/// ```
#[inline(always)]
pub fn get_scale(ctx: &Context) -> Vec2 {
    let (width, height) = ctx.gfx.drawable_size();
    Vec2::new(width / 1920., height / 1080.)
}

/// Returns if the player would collide with a machine if they moved in the given direction
#[inline(always)]
pub fn is_colliding(player_pos: (usize, usize), area: &Rect) -> bool {
    max(area.x as usize, player_pos.0) <= min((area.x + area.w) as usize, player_pos.0 + 100)
        && max(area.y as usize, player_pos.1) <= min((area.y + area.h) as usize, player_pos.1 + 100)
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
