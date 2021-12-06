use iced::{button, executor, Application, Button, Column, Command, Element, Settings, Text};
use reqwest;

pub fn main() -> iced::Result {
    State::run(Settings::default())
}

#[derive(Default)]
struct State {
    value: String,
    request_button: button::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    RequestPressed,
}

impl Application for State {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("GraphQL Request - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::RequestPressed => {
                let result = reqwest::blocking::get("https://jsonplaceholder.typicode.com/posts/");
                let result = match result {
                    Ok(response) => response.text().unwrap(),
                    Err(e) => {
                        panic!("{:?}", e);
                    }
                };
                self.value = result.to_string();
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .push(Text::new(self.value.to_string()).size(50))
            .push(
                Button::new(&mut self.request_button, Text::new("Request"))
                    .on_press(Message::RequestPressed),
            )
            .into()
    }
}
