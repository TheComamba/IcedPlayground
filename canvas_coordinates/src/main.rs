use iced::{
    alignment::{Horizontal, Vertical},
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
        let vertical_offset = Text::new("Vertical Offset")
            .height(iced::Length::Fixed(250.))
            .vertical_alignment(Vertical::Center)
            .horizontal_alignment(Horizontal::Center);
        let horizontal_offset = Text::new("Horizontal Offset")
            .width(iced::Length::Fixed(250.))
            .vertical_alignment(Vertical::Center)
            .horizontal_alignment(Horizontal::Center);

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

        let coordinates = self
            .coordinates_cache
            .draw(renderer, bounds.size(), |frame| {
                let center_of_frame = frame.center();
                let dot = Path::circle(center_of_frame, 5.);
                frame.fill(&dot, Color::WHITE);
                let mut center_of_frame_text = canvas::Text::default();
                center_of_frame_text.content = "Center of Frame".to_string();
                center_of_frame_text.position = center_of_frame;
                center_of_frame_text.color = Color::WHITE;
                center_of_frame_text.vertical_alignment = Vertical::Top;
                frame.fill_text(center_of_frame_text);

                let center_of_bounds = Point::new(bounds.width / 2., bounds.height / 2.);
                let dot = Path::circle(center_of_bounds, 5.);
                frame.fill(&dot, Color::WHITE);
                let mut center_of_bounds_text = canvas::Text::default();
                center_of_bounds_text.content = "Center of Bounds".to_string();
                center_of_bounds_text.position = center_of_frame;
                center_of_bounds_text.color = Color::WHITE;
                center_of_bounds_text.vertical_alignment = Vertical::Bottom;
                frame.fill_text(center_of_bounds_text);

                for x in 1..10 {
                    for y in 1..10 {
                        let x = x as f32 * 100.;
                        let y = y as f32 * 100.;
                        let point = Point::new(x, y);
                        let dot = Path::circle(point, 2.);
                        frame.fill(&dot, Color::WHITE);
                        let mut text = canvas::Text::default();
                        text.content = format!("({}, {})", x, y);
                        text.position = point;
                        text.color = Color::WHITE;
                        text.size = 15.;
                        if !bounds.contains(point) {
                            text.content = format!("({}, {})\nNot inside\nbounds", x, y);
                            text.color.a = 0.5;
                        }
                        frame.fill_text(text);
                    }
                }
            });

        vec![background, coordinates]
    }
}
