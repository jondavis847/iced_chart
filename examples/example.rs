use iced::Element;
use iced_chart::plot::Plot;

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
        // Create a plot with the current theme
        let plot = Plot {
            size: Size::new(400.0, 300.0),
            width: 400.0,
            height: 300.0,
            background: Color::from_rgb(0.1, 0.1, 0.1),
        };

        iced::widget::container(plot)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .border_radius(5)
            .style(border::Style::Solid)
    }
}

fn main() -> iced::Result {
    iced::run("plot_example", Example::update, Example::view)
}
