use iced::{
    widget::{
        scrollable::{Direction, Properties},
        Column, Rule, Scrollable, Text,
    },
    Sandbox,
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
        "Rule on Scrollable".to_string()
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let rule_direction = Direction::Horizontal(Properties::default());

        let simple_rule = Column::new()
            .push(Text::new("A simple Rule without a Scrollable is ok."))
            .push(Rule::horizontal(20.));

        let simple_scrollable = Column::new().push(
            Scrollable::new(Text::new(
                "A simple Scrollable without any Rule is happy as well.",
            ))
            .direction(rule_direction)
            .width(iced::Length::Fixed(300.)),
        );

        let rule_on_scrollable_fixed = Column::new()
            .push(Text::new(
                "A lonely Rule on a Scrollable with fixed width is invisible, while the Scrollable content is unfathomably large (note the tiny scrollbar). Additionally, this text has linebreaks instead of being scrollable.",
            ))
            .push(Scrollable::new(Rule::horizontal(20.)).direction(rule_direction))
            .width(iced::Length::Fixed(300.));

        let rule_on_scrollable = Column::new()
            .push(Text::new(
                "The situation is the same if the Scrollable has a Fill width. The Scrollable content is unfathomably large, and this text has linebreaks instead of being scrollable.",
            ))
            .push(Scrollable::new(Rule::horizontal(20.)).direction(rule_direction))
            .width(iced::Length::Fill);

        Column::new()
            .push(simple_rule)
            .push(simple_scrollable)
            .push(rule_on_scrollable_fixed)
            .push(rule_on_scrollable)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .spacing(20.)
            .padding(20.)
            .into()
    }
}
