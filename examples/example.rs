use iced::{Color, Element, Length, Padding};
use iced_chart::{axes::Axes, plot::Plot};

#[derive(Debug)]
enum Message {
    ChangeTheme,
}

#[derive(Default)]
struct Example {}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::ChangeTheme => {
                // Change the theme of the plot
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let mut plot = Plot::new()
            .with_width(Length::Fill)
            .with_height(Length::Fill)
            .with_padding(Padding::from(10))
            .with_background_color(Color::from_rgb(0.05, 0.05, 0.05))
            .with_border_color(Color::from_rgb(0.2, 0.2, 0.2));
        plot.add_axes(
            Axes::new(0, 0)
                .with_background_color(Color::from_rgb(0.1, 0.1, 0.1))
                .with_border_color(Color::from_rgb(0.2, 0.2, 0.2)),
        );
        plot.add_axes(
            Axes::new(0, 1)
                .with_background_color(Color::from_rgb(0.1, 0.1, 0.1))
                .with_border_color(Color::from_rgb(0.2, 0.2, 0.2)),
        );
        plot.add_axes(
            Axes::new(1, 0)
                .with_background_color(Color::from_rgb(0.1, 0.1, 0.1))
                .with_border_color(Color::from_rgb(0.2, 0.2, 0.2)),
        );
        plot.add_axes(
            Axes::new(1, 1)
                .with_background_color(Color::from_rgb(0.1, 0.1, 0.1))
                .with_border_color(Color::from_rgb(0.2, 0.2, 0.2)),
        );
        Element::from(plot)
    }
}

fn main() -> iced::Result {
    iced::run("plot_example", Example::update, Example::view)
}
