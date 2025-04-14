use std::sync::Arc;

use iced::{
    Border, Color, Element, Length, Padding, Rectangle, Size,
    advanced::{
        Layout, Widget,
        layout::{self, Limits, Node},
        renderer::{self, Quad, Style},
        widget::Tree,
    },
    border,
    mouse::Cursor,
};
pub struct Plot<'a, Message, Renderer> {
    content: Element<'a, Message, Renderer>,
    width: Length,
    height: Length,
    padding: Padding,
    background: Color,
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Plot<'a, Message, Renderer>
where
    Renderer: iced::advanced::renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(&self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        layout::padded(limits, self.width, self.height, self.padding, |limits| {
            self.content
                .as_widget()
                .layout(&mut tree.children[0], renderer, limits)
        })
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
