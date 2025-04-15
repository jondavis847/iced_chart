use iced::{Background, Border, Color, Padding};

pub struct Axis {
    padding: Padding,
    background: Background,
    border: Border,
}

impl Default for Axis {
    fn default() -> Self {
        Axis {
            padding: Padding::new(30.0),
            background: Background::from(Color::from_rgb(0.1, 0.1, 0.1)),
            border: Border::default(),
        }
    }
}
