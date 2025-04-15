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
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(Padding::from(10))
            .background(Color::from_rgb(0.05, 0.05, 0.05));
        plot.add_axes(Axes::new(0, 0).with_background_color(Color::from_rgb(0.9, 0.1, 0.1)));
        plot.add_axes(Axes::new(0, 1).with_background_color(Color::from_rgb(0.1, 0.9, 0.1)));
        plot.add_axes(Axes::new(1, 0).with_background_color(Color::from_rgb(0.1, 0.1, 0.9)));
        plot.add_axes(Axes::new(1, 1).with_background_color(Color::from_rgb(0.9, 0.9, 0.9)));
        Element::from(plot)
    }
}

fn main() -> iced::Result {
    iced::run("plot_example", Example::update, Example::view)
}
