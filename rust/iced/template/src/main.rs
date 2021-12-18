use iced::image::Viewer;
use iced::Row;
use iced::{
    button, executor, scrollable, Application, Button, Clipboard, Column, Command, Element,
    Scrollable, Settings, Text,
};

pub fn main() -> iced::Result {
    State::run(Settings::default())
}

#[derive(Debug)]
enum App {
    Loading,
    Loaded(State),
    Error,
}

#[derive(Default, Debug)]
struct State {
    scroll: scrollable::State,
    value: String,
    button: button::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    ButtonPressed,
}

impl Application for State {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Iced - Template")
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::ButtonPressed => {
                self.value = "clicked!".to_string();
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let text = Text::new(&self.value).size(10);
        let button =
            Button::new(&mut self.button, Text::new("Request")).on_press(Message::ButtonPressed);
        let content = Column::new()
            .padding(20)
            .spacing(20)
            .max_width(500)
            .push(text)
            .push(button);

        Scrollable::new(&mut self.scroll).push(content).into()
    }
}

impl App {
    fn view(&mut self) -> Element<Message> {
        Row::new().spacing(20).into()
    }
}
