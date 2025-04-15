use iced::{Color, Padding};

use crate::{axis::Axis, grid::Grid, legend::Legend};

pub struct Axes {
    axis: Axis,
    background_color: Color,
    grid: Grid,
    legend: Legend,
    padding: Padding,
}

impl Axes {
    pub fn new() -> Self {
        Self {
            axis: Axis::new(),
            background_color: Color::WHITE,
            grid: Grid::new(),
            legend: Legend::new(),
            padding: Padding::from(10),
        }
    }
}
