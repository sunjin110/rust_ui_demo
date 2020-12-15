
use iced::Application;

struct GUI;

impl iced::Application for GUI {
    type Executor = iced::executor::Null;
    type Message = ();
    type Flags = ();

    fn new(_flags: ()) -> (GUI, iced::Command<Self::Message>) {

        // 初期化の時点でやりたい処理がないためnone
        (GUI, iced::Command::none())
    }

    fn title(&self) -> String {
        String::from("DEMO")
    }

    fn update(&mut self, _message: Self::Message) -> iced::Command<Self::Message> {
        iced::Command::none()
    }

    fn view(&mut self) -> iced::Element<Self::Message> {
        iced::Text::new("Hello, World").into()
    }
}

fn main() {
    GUI::run(iced::Settings::default());
}
