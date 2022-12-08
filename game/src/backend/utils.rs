//! Miscellaneous utilities used by the backend.
use crate::backend::constants::{PLAYER_ICON_SIZE, SCREEN_RESOLUTION};
use ggez::glam::Vec2;
use ggez::graphics::{Color, Rect};
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

/// Returns if the player collides with an area
/// # Arguments
/// * `player_pos` - The position of the player
/// * `direction` - The direction the player wants to move
/// # Returns
/// * `true` if the player collides with an area
/// * `false` if the player does not collide with an area
#[inline(always)]
pub fn is_colliding(player_pos: (usize, usize), area: &Rect) -> bool {
    area.x < player_pos.0 as f32 + PLAYER_ICON_SIZE.0 as f32
        && area.x + area.w > player_pos.0 as f32
        && area.y < player_pos.1 as f32 + PLAYER_ICON_SIZE.1 as f32
        && area.y + area.h > player_pos.1 as f32
}

/// This macro is used for simplifying drawing with scaling.
/// It takes a canvas, a `Drawable`, an (optional) position(as `Vec2` for example) and a scale as `Vec2`.
#[macro_export]
macro_rules! draw {
    ($canvas: expr, $asset: expr, $position: expr, $scale: expr) => {
        $canvas.draw($asset, get_draw_params(Some($position), $scale, None))
    };
    ($canvas: expr, $drawable: expr, $scale: expr) => {
        $canvas.draw($drawable, get_draw_params(None, $scale, None))
    };
    ($canvas: expr, $asset: expr, $position: expr, $scale: expr, $color: expr) => {
        $canvas.draw($asset, get_draw_params($position, $scale, $color))
    };
}
/// Used in the draw macro to get the draw parameters
/// # Arguments
/// * `position` - The optional position of the asset
/// * `scale` - The scale of the asset
/// * `color` - The optional color of the asset
/// # Returns
/// The draw parameters
pub fn get_draw_params(
    position: Option<Vec2>,
    scale: Vec2,
    color: Option<Color>,
) -> ggez::graphics::DrawParam {
    let mut param = ggez::graphics::DrawParam::new().scale(scale);
    if let Some(pos) = position {
        param = param.dest(pos * scale);
    }
    if let Some(col) = color {
        param = param.color(col);
    }
    param
}
