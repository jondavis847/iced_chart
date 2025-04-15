use std::collections::HashMap;

use iced::{
    Color, Element, Length, Padding, Rectangle, Size,
    advanced::{
        Layout, Widget,
        layout::{Limits, Node},
        renderer::{self, Quad, Style},
        widget::{Id, Tree},
    },
    mouse::Cursor,
};

use crate::{PlotErrors, axes::Axes};
pub struct Plot {
    width: Length,
    height: Length,
    padding: Padding,
    background_color: Color,
    border_color: Color,
    axes: HashMap<Id, Axes>,
}

impl Plot {
    pub fn new() -> Self {
        Self {
            width: Length::Fill,
            height: Length::Fill,
            padding: Padding::ZERO,
            background_color: Color::WHITE,
            border_color: Color::BLACK,
            axes: HashMap::new(),
        }
    }

    pub fn with_width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    pub fn with_height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    pub fn with_padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn with_background_color(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }

    pub fn with_border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }

    pub fn add_axes(&mut self, axes: Axes) {
        self.axes.insert(axes.id.clone(), axes);
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Plot
where
    Renderer: iced::advanced::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(&self, _tree: &mut Tree, _renderer: &Renderer, limits: &Limits) -> Node {
        let size = limits.resolve(self.width, self.height, Size::ZERO);

        Node::new(size)
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        renderer.fill_quad(
            Quad {
                bounds: layout.bounds(),
                ..renderer::Quad::default()
            },
            self.background_color,
        );

        // Calculate cell size based on the number of rows and columns
        let total_rows = self
            .axes
            .iter()
            .map(|(_, axes)| axes.row)
            .max()
            .unwrap_or(0)
            + 1;
        let total_cols = self
            .axes
            .iter()
            .map(|(_, axes)| axes.column)
            .max()
            .unwrap_or(0)
            + 1;
        let plot_bounds = layout.bounds();

        let cell_width = plot_bounds.width / total_cols as f32;
        let cell_height = plot_bounds.height / total_rows as f32;

        // Draw each Axes
        for axes in self.axes.values() {
            let x = plot_bounds.x + axes.column as f32 * cell_width + axes.padding.left as f32;
            let y = plot_bounds.y + axes.row as f32 * cell_height + axes.padding.top as f32;

            let width = cell_width - (axes.padding.left + axes.padding.right) as f32;
            let height = cell_height - (axes.padding.top + axes.padding.bottom) as f32;

            let bounds = Rectangle {
                x,
                y,
                width,
                height,
            };
            axes.draw(renderer, bounds);
        }
    }
}

impl<'a, Message, Theme, Renderer> From<Plot> for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: 'a + iced::advanced::Renderer,
{
    fn from(plot: Plot) -> Self {
        Element::new(plot)
    }
}
