use iced::Color;

pub enum PlotThemes {
    Dark,
    Light,
}

impl PlotThemes {
    pub fn get_theme(&self) -> PlotTheme {
        match self {
            PlotThemes::Dark => PlotTheme {
                plot_background: Color::from_rgb(0.1, 0.1, 0.1),
            },
            PlotThemes::Light => PlotTheme {
                plot_background: Color::from_rgb(1.0, 1.0, 1.0),
            },
        }
    }
}

pub struct PlotTheme {
    plot_background: Color,
}
