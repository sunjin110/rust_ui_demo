use iced::Application;

const FPS: u64 = 30;
const MILLISEC: u64 = 1000;
const MINUTE: u64 = 60;
const HOUR: u64 = 60 * MINUTE;

struct GUI {
    last_update: std::time::Instant,
    total_duration: std::time::Duration,
    tick_state: TickState,
    start_stop_button_state: iced::button::State,
    reset_button_state: iced::button::State,
}

// イベント送信用のメッセージ
#[derive(Debug, Clone)]
pub enum Message {
    Start, // 時間の測定を開始するメッセージ
    Stop,  // 時間の測定を停止するメッセージ
    Reset, // 測定した時間をリセットするメッセージ
    Update, 
}

// 状態(シーン的な？)
pub enum TickState {
    Stopped,
    Ticking,
}

// timer
pub struct Timer {
    duration: std::time::Duration,
}

impl Timer {
    fn new(duration: std::time::Duration) -> Timer {
        Timer { duration: duration}
    }
}

impl <H, E> iced_native::subscription::Recipe<H, E> for Timer where H: std::hash::Hasher {
    type Output = std::time::Instant;

    fn hash(&self, state: &mut H) {
        use std::hash::Hash;
        std::any::TypeId::of::<Self>().hash(state);
        self.duration.hash(state);
    }

    fn stream(
        self: Box<Self>,
        _input: iced_futures::futures::stream::BoxStream<'static, E>,
    ) -> iced_futures::futures::stream::BoxStream<'static, Self::Output> {
        use iced_futures::futures::stream::StreamExt;
        async_std::stream::interval(self.duration)
            .map(|_| std::time::Instant::now())
            .boxed()
    }
}

impl iced::Application for GUI {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (GUI, iced::Command<Self::Message>) {
        (
            GUI {
                tick_state: TickState::Stopped,
                start_stop_button_state: iced::button::State::new(),
                reset_button_state: iced::button::State::new(),
            },
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("DEMO")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Start => {
                self.tick_state = TickState::Ticking;
            }
            Message::Stop => {
                self.tick_state = TickState::Stopped;
            }
            Message::Reset => {}
            Message::Update => {}
        }

        iced::Command::none()
    }

    fn view(&mut self) -> iced::Element<Self::Message> {
        // prepare duration text
        let duration_text = "00:00:00:00";

        // prepare start/stop text
        let start_stop_text = match self.tick_state {
            TickState::Stopped => iced::Text::new("Start")
                .horizontal_alignment(iced::HorizontalAlignment::Center)
                .font(iced::Font::Default),
            TickState::Ticking => iced::Text::new("Stop")
                .horizontal_alignment(iced::HorizontalAlignment::Center)
                .font(iced::Font::Default),
        };

        // prepare start/stop message on button press
        let start_stop_message = match self.tick_state {
            TickState::Stopped => Message::Start,
            TickState::Ticking => Message::Stop,
        };

        
        // init widgets
        let tick_text = iced::Text::new(duration_text)
            .font(iced::Font::Default)
            .size(60);

        let start_stop_button = iced::Button::new(
            &mut self.start_stop_button_state,
            start_stop_text,
        )
        .min_width(80)
        .on_press(start_stop_message);

        let reset_button = iced::Button::new(
            &mut self.reset_button_state,
            iced::Text::new("Reset")
                .horizontal_alignment(iced::HorizontalAlignment::Center)
                .font(iced::Font::Default),
        )
        .min_width(80)
        .on_press(Message::Reset);

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

    fn subscription(&self) -> iced::Subscription<Message> {
        let timer = Timer::new(std::time::Duration::from_millis(MILLISEC / FPS));
        iced::Subscription::from_recipe(timer).map(|_| Message::Update)
    }
}

fn main() {
    // 画面サイズを変更
    let mut settings = iced::Settings::default();
    settings.window.size = (400u32, 120u32);
    GUI::run(settings);
}
