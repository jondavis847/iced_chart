use iced::advanced::widget::Id;
use iced::widget::canvas::Frame;
// In axes.rs
use iced::{Background, Border, Padding};
use iced::{Color, Point, Rectangle, Size, Vector, advanced::renderer::Renderer};

pub struct Axes {
    pub id: Id,
    pub row: usize,
    pub column: usize,
    pub padding: Padding,
    // Data properties
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
    // Style properties
    pub background: Background,
    pub border: Border,
    pub axis_color: Color,
    pub line_width: f32,
    pub show_grid: bool,
    pub title: String,
    pub x_label: String,
    pub y_label: String,
}

impl Axes {
    pub fn new(row: usize, column: usize) -> Self {
        Self {
            id: Id::unique(),
            row,
            column,
            padding: Padding::from(10),
            x_min: 0.0,
            x_max: 10.0,
            y_min: 0.0,
            y_max: 10.0,
            background: Color::from_rgb(0.1, 0.1, 0.1).into(),
            border: Border::default(),
            axis_color: Color::BLACK,
            line_width: 1.0,
            show_grid: false,
            title: String::new(),
            x_label: String::new(),
            y_label: String::new(),
        }
    }

    pub fn with_background(mut self, background: Background) -> Self {
        self.background = background;
        self
    }

    pub fn with_border(mut self, border: Border) -> Self {
        self.border = border;
        self
    }

    // Builder pattern methods for configuration
    pub fn with_range(mut self, x_min: f32, x_max: f32, y_min: f32, y_max: f32) -> Self {
        self.x_min = x_min;
        self.x_max = x_max;
        self.y_min = y_min;
        self.y_max = y_max;
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_axis_labels(
        mut self,
        x_label: impl Into<String>,
        y_label: impl Into<String>,
    ) -> Self {
        self.x_label = x_label.into();
        self.y_label = y_label.into();
        self
    }

    // This is used by the Plot widget to draw this Axes
    pub fn draw(&self, frame: &mut Frame) {}
}
