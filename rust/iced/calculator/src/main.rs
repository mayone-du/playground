use iced::{text_input, Application, Button, Column, Command, Settings, TextInput};

pub fn main() -> iced::Result {
    State::run(Settings::default())
}

#[derive(Debug, Default)]
struct State {
    value: i32,
    current_input: text_input::State,
}

#[derive(Debug, Clone)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Reset,
}

#[derive(Debug, Clone)]
enum Message {
    ButtonPressed(Operation),
    CurrentInputChanged(i32),
}

impl Application for State {
    type Message = Message;

    fn new(_flags: ()) -> (Command<Message>) {
        (Command::perform({}, Message::ButtonPressed(Operation::Reset)))
    }
}
