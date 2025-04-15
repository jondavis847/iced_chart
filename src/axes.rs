use crate::{axis::Axis, grid::Grid, legend::Legend};
use iced::{
    Border, Color, Padding, Rectangle,
    advanced::{
        renderer::{self, Quad},
        widget::Id,
    },
};

pub struct Axes {
    pub id: Id,
    axis: Axis,
    background_color: Color,
    border_color: Color,
    grid: Grid,
    legend: Legend,
    pub padding: Padding,
    pub row: usize,
    pub column: usize,
}

impl Axes {
    pub fn new(row: usize, column: usize) -> Self {
        Self {
            id: Id::unique(),
            axis: Axis::new(),
            background_color: Color::WHITE,
            border_color: Color::BLACK,
            grid: Grid::new(),
            legend: Legend::new(),
            padding: Padding::from(10),
            row,
            column,
        }
    }

    pub fn with_background_color(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }
    pub fn with_border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }

    pub fn draw<Renderer>(&self, renderer: &mut Renderer, bounds: Rectangle)
    where
        Renderer: renderer::Renderer,
    {
        renderer.fill_quad(
            Quad {
                bounds,
                border: Border {
                    color: self.border_color,
                    width: 1.0,
                    radius: 0.0.into(),
                },
                shadow: Default::default(),
            },
            self.background_color,
        );
    }
}
