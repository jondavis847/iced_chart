use iced::{Color, Padding};

use crate::axis::Axis;

pub struct AxesContainer {
    background_color: Color,
    axes: Axes,
    padding: Padding,
}

pub struct Axes {
    axis: Axis,
}
