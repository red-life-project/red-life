use ggez::glam::Vec2;
use ggez::Context;
/// This returns the scale so we can have resolution-agnostic scaling
/// Use it in your drawing calls like this:
/// ```rust
/// let scale = get_scale(ctx);
/// graphics::draw(ctx, &self.img, graphics::DrawParam::default().scale(scale))?;
/// ```
pub fn get_scale(ctx: &Context) -> Vec2 {
    let (width, height) = ctx.gfx.drawable_size();
    Vec2::new(width / 1920., height / 1080.)
}
#[macro_export]
/// This macro is used for simplifying drawing with scaling.
/// It takes a canvas, a `Drawable`, an (optional) position(as `Vec2` for example) and a scale.
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
