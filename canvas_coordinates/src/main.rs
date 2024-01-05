use iced::{
    widget::{canvas, canvas::Path, Column, Row, Text},
    Color, Point, Sandbox,
};

fn main() -> iced::Result {
    Gui::run(iced::Settings::default())
}

struct Gui {
    background_cache: canvas::Cache,
    coordinates_cache: canvas::Cache,
}

impl Sandbox for Gui {
    type Message = ();

    fn new() -> Self {
        Gui {
            background_cache: canvas::Cache::default(),
            coordinates_cache: canvas::Cache::default(),
        }
    }

    fn title(&self) -> String {
        "Canvas Coordinates".to_string()
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let vertical_offset = Text::new("Vertical Offset").height(iced::Length::Fixed(200.));
        let horizontal_offset = Text::new("Horizontal Offset").width(iced::Length::Fixed(200.));

        Column::new()
            .push(vertical_offset)
            .push(
                Row::new().push(horizontal_offset).push(
                    canvas(self)
                        .width(iced::Length::Fill)
                        .height(iced::Length::Fill),
                ),
            )
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}

impl<GuiMessage> canvas::Program<GuiMessage> for Gui {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::theme::Theme,
        bounds: iced::Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let background = self
            .background_cache
            .draw(renderer, bounds.size(), |frame| {
                let background = Path::rectangle(Point::ORIGIN, bounds.size());
                frame.fill(&background, Color::BLACK);
            });

        vec![background]
    }
}
