use ggez::graphics::Color;

pub struct RLColor {}

impl RLColor {
    pub const GREY: Color = Color::from_rgba(195, 195, 195, 255);
    pub const BLUE: Color = Color::from_rgba(51, 51, 204, 255);
    pub const GOLD: Color = Color::from_rgba(186, 158, 19, 255);
    pub const RED: Color = Color::from_rgba(102, 24, 18, 255);
}