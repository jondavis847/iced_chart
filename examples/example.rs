use iced::{Color, Element, Length, Padding};
use iced_chart::plot::Plot;

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
        let plot = Plot::new(iced::widget::text("Plot Content").into())
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(Padding::from(10))
            .background(Color::from_rgb(0.1, 0.1, 0.1));

        Element::new(plot)
    }
}

fn main() -> iced::Result {
    iced::run("plot_example", Example::update, Example::view)
}
