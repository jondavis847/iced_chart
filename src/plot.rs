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

use crate::axes::Axes;
pub struct Plot {
    width: Length,
    height: Length,
    padding: Padding,
    background: Color,
    axes: HashMap<Id, Axes>,
}

impl Plot {
    pub fn new() -> Self {
        Self {
            width: Length::Fill,
            height: Length::Fill,
            padding: Padding::ZERO,
            background: Color::WHITE,
            axes: HashMap::new(),
        }
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn background(mut self, background: Color) -> Self {
        self.background = background;
        self
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
            self.background,
        )
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
