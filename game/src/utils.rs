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
