use std::collections::HashMap;

use crate::axes::Axes;
use iced::{
    Color, Element, Length, Padding, Rectangle, Renderer, Size, Theme,
    advanced::{
        Layout, Widget,
        layout::{Limits, Node},
        renderer::{self, Quad, Style},
        widget::{Id, Tree},
    },
    event::Status,
    mouse::{Cursor, Interaction},
    widget::{
        Canvas,
        canvas::{Cache, Program},
    },
};

#[derive(Default)]
pub struct PlotState {
    cache: Cache,
}

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

impl<Message> Program<Message> for Plot {
    type State = PlotState;
    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        cursor: iced::advanced::mouse::Cursor,
    ) -> Vec<iced::widget::canvas::Geometry<Renderer>> {
        let content = state.cache.draw(renderer, bounds.size(), |frame| {
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

            let cell_width = bounds.width / total_cols as f32;
            let cell_height = bounds.height / total_rows as f32;

            // Draw each Axes
            for axes in self.axes.values() {
                let x = bounds.x + axes.column as f32 * cell_width + axes.padding.left as f32;
                let y = bounds.y + axes.row as f32 * cell_height + axes.padding.top as f32;

                let width = cell_width - (axes.padding.left + axes.padding.right) as f32;
                let height = cell_height - (axes.padding.top + axes.padding.bottom) as f32;

                let axes_bounds = Rectangle {
                    x,
                    y,
                    width,
                    height,
                };

                frame.with_clip(axes_bounds, |frame| {
                    axes.draw(frame);
                });
            }
        });
        vec![content]
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        _bounds: Rectangle,
        _cursor: iced::advanced::mouse::Cursor,
    ) -> Interaction {
        Interaction::None
    }

    fn update(
        &self,
        _state: &mut Self::State,
        _event: iced::widget::canvas::Event,
        _bounds: Rectangle,
        _cursor: iced::advanced::mouse::Cursor,
    ) -> (Status, Option<Message>) {
        (Status::Captured, None)
    }
}
