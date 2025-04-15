use crate::{axis::Axis, grid::Grid, legend::Legend};
use iced::{
    Background, Border, Color, Padding, Rectangle,
    advanced::{
        renderer::{self, Quad},
        widget::Id,
    },
};

pub struct Axes {
    pub id: Id,
    axis: Axis,
    background: Background,
    border: Border,
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
            axis: Axis::default(),
            background: Background::from(Color::WHITE),
            border: Border::default(),
            grid: Grid::new(),
            legend: Legend::new(),
            padding: Padding::from(10),
            row,
            column,
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

    pub fn draw<Renderer>(&self, renderer: &mut Renderer, bounds: Rectangle)
    where
        Renderer: renderer::Renderer,
    {
        renderer.fill_quad(
            Quad {
                bounds,
                border: self.border,
                shadow: Default::default(),
            },
            self.background,
        );
    }
}
