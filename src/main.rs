use iced::Application;

struct GUI {
    start_stop_button_state: iced::button::State,
    reset_button_state: iced::button::State,
}

impl iced::Application for GUI {
    type Executor = iced::executor::Null;
    type Message = ();
    type Flags = ();

    fn new(_flags: ()) -> (GUI, iced::Command<Self::Message>) {
        (
            GUI {
                start_stop_button_state: iced::button::State::new(),
                reset_button_state: iced::button::State::new(),
            },
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("DEMO")
    }

    fn update(&mut self, _message: Self::Message) -> iced::Command<Self::Message> {
        iced::Command::none()
    }

    fn view(&mut self) -> iced::Element<Self::Message> {

        // init widgets
        let tick_text = iced::Text::new("00:00:00:00")
            .font(iced::Font::Default)
            .size(60);

        let start_stop_button = iced::Button::new(
            &mut self.start_stop_button_state,
            iced::Text::new("Start")
                .horizontal_alignment(iced::HorizontalAlignment::Center)
                .font(iced::Font::Default),
        )
        .min_width(80);

        let reset_button = iced::Button::new(
            &mut self.reset_button_state,
            iced::Text::new("Reset")
                .horizontal_alignment(iced::HorizontalAlignment::Center)
                .font(iced::Font::Default),
        )
        .min_width(80);

        // prepare column
        iced::Column::new().push(tick_text).push(
            iced::Row::new()
                .push(start_stop_button)
                .push(reset_button)
                .spacing(10),
        )
        .spacing(10)
        .padding(10)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .align_items(iced::Align::Center)
        .into()
    }
}

fn main() {
    GUI::run(iced::Settings::default());
}
