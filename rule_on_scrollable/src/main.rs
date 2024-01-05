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
            .push(Text::new("Simple Rule is ok"))
            .push(Rule::horizontal(20.));

        let rule_on_scrollable = Column::new()
            .push(Text::new("A lonely Rule on a Scrollable is invisible"))
            .push(Scrollable::new(Rule::horizontal(20.)).direction(rule_direction));

        let rule_and_text = Column::new()
            .push(Text::new("Rule and Text on Scrollable is ok"))
            .push(Rule::horizontal(20.));
        let rule_and_text_on_scrollable = Scrollable::new(rule_and_text);

        let table_header = Text::new(
            "A rather lengthy text to demonstrate that the scrollabel does or does not appear. Again: A rather lengthy text to demonstrate that the scrollabel does or does not appear.",
        );
        let table = Column::new().push(table_header); //.push(Rule::horizontal(10));
        let direction = Direction::Both {
            vertical: Properties::default(),
            horizontal: Properties::default(),
        };
        let bla = Scrollable::new(table)
            .direction(direction)
            .width(iced::Length::Fill);

        Column::new()
            .push(simple_rule)
            .push(rule_on_scrollable)
            .push(rule_and_text_on_scrollable)
            .push(bla)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}
