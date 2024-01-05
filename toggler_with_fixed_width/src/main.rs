use iced::{
    widget::{Column, Text, Toggler},
    Length, Sandbox,
};

fn main() -> iced::Result {
    Gui::run(iced::Settings::default())
}

struct Gui {}

impl Sandbox for Gui {
    type Message = ();

    fn new() -> Self {
        Gui {}
    }

    fn title(&self) -> String {
        "Toggler with fixed width".to_string()
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> iced::Element<'_, Self::Message> {
        Column::new()
            .push(Text::new("Toggler without any specifications:"))
            .push(Toggler::new(None, true, |_| ()))
            .push(Text::new("Toggler with text:"))
            .push(Toggler::new(Some("Text".to_string()), true, |_| ()))
            .push(Text::new("Toggler with fixed width:"))
            .push(Toggler::new(None, true, |_| ()).width(Length::Fixed(100.)))
            .push(Text::new("Toggler with text and fixed width:"))
            .push(Toggler::new(Some("Text".to_string()), true, |_| ()).width(Length::Fixed(100.)))
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .spacing(20.)
            .padding(20.)
            .into()
    }
}
