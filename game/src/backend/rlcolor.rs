//! Contains the color definitions for the game. Every color is defined as a constant.
use good_web_game::graphics::Color;
/// Storing colors used in the game as constants
pub struct RLColor {}

impl RLColor {
    /// Color: light grey
    pub const LIGHT_GREY: Color = Color {
        r: 0.49,
        g: 0.49,
        b: 0.49,
        a: 0.8,
    };
    /// Color: grey
    pub const GREY: Color = Color {
        r: 0.765,
        g: 0.765,
        b: 0.765,
        a: 1.,
    };
    /// Color: dark grey
    pub const DARK_GREY: Color = Color {
        r: 0.58,
        g: 0.575,
        b: 0.561,
        a: 1.,
    };
    /// Color: light blue
    pub const LIGHT_BLUE: Color = Color {
        r: 0.039,
        g: 0.039,
        b: 1.,
        a: 1.,
    };
    /// Color: blue
    pub const BLUE: Color = Color {
        r: 0.2,
        g: 0.2,
        b: 0.8,
        a: 1.,
    };
    /// Color: dark blue
    pub const DARK_BLUE: Color = Color {
        r: 0.1,
        g: 0.2,
        b: 0.3,
        a: 1.,
    };
    /// Color: gold
    pub const GOLD: Color = Color {
        r: 0.73,
        g: 0.62,
        b: 0.075,
        a: 1.,
    };
    /// Color: red
    pub const RED: Color = Color {
        r: 1.,
        g: 0.039,
        b: 0.039,
        a: 1.,
    };
    /// Color: dark red
    pub const DARK_RED: Color = Color {
        r: 0.4,
        g: 0.094,
        b: 0.076,
        a: 1.,
    };
    /// Color: green
    pub const GREEN: Color = Color {
        r: 0.2,
        g: 0.922,
        b: 0.2,
        a: 1.,
    };
    /// Color: black
    pub const BLACK: Color = Color {
        r: 0.,
        g: 0.,
        b: 0.,
        a: 1.,
    };
    /// Color: green (status)
    pub const STATUS_GREEN: Color = Color {
        r: 0.,
        g: 1.,
        b: 0.,
        a: 1.,
    };
    /// Color: yellow (status)
    pub const STATUS_YELLOW: Color = Color {
        r: 1.,
        g: 1.,
        b: 0.,
        a: 1.,
    };
    /// Color: red (status)
    pub const STATUS_RED: Color = Color {
        r: 1.,
        g: 0.,
        b: 0.,
        a: 1.,
    };
}
