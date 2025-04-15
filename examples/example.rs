use iced::{Border, Color, Element, Length, Padding, border::Radius};
use iced_chart::{axes::Axes, plot::Plot};

#[derive(Debug)]
enum Message {
    ChangeTheme,
}

struct Example {
    plot: Plot,
}

impl Default for Example {
    fn default() -> Self {
        let mut plot = Plot::new()
            .with_width(Length::Fill)
            .with_height(Length::Fill)
            .with_padding(Padding::from(10))
            .with_background_color(Color::from_rgb(0.05, 0.05, 0.05))
            .with_border_color(Color::from_rgb(0.2, 0.2, 0.2));

        let axes_border = Border {
            color: Color::from_rgb(0.2, 0.2, 0.2),
            width: 1.0,
            radius: Radius::new(10.0),
        };

        plot.add_axes(
            Axes::new(0, 0)
                .with_background(Color::from_rgb(0.1, 0.1, 0.1).into())
                .with_border(axes_border),
        );
        plot.add_axes(
            Axes::new(0, 1)
                .with_background(Color::from_rgb(0.1, 0.1, 0.1).into())
                .with_border(axes_border),
        );
        plot.add_axes(
            Axes::new(1, 0)
                .with_background(Color::from_rgb(0.1, 0.1, 0.1).into())
                .with_border(axes_border),
        );
        plot.add_axes(
            Axes::new(1, 1)
                .with_background(Color::from_rgb(0.1, 0.1, 0.1).into())
                .with_border(axes_border),
        );
        Self { plot }
    }
}
impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::ChangeTheme => {
                // Change the theme of the plot
            }
        }
    }

    fn view(&self) -> Element<Message> {
        self.plot.canvas
    }
}

fn main() -> iced::Result {
    iced::run("plot_example", Example::update, Example::view)
}
