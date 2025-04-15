use iced::{Background, Border, Color, Padding, widget::canvas::Frame};

pub struct Axis {
    padding: Padding,
    background: Background,
    border: Border,
    limits: (f64, f64),
}

impl Default for Axis {
    fn default() -> Self {
        Axis {
            padding: Padding::new(30.0),
            background: Background::from(Color::from_rgb(0.1, 0.1, 0.1)),
            border: Border::default(),
            limits: (-1.0, 1.0),
        }
    }
}

impl Axis {
    pub fn with_padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn with_background(mut self, background: Background) -> Self {
        self.background = background;
        self
    }

    pub fn with_border(mut self, border: Border) -> Self {
        self.border = border;
        self
    }

    pub fn draw(&self, frame: &mut Frame) {
        // horizontal axis
    }
}
