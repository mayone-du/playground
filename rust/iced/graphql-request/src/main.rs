use iced::{button, Button, Column, Element, Sandbox, Settings, Text};
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

impl Sandbox for State {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("GraphQL Request - Iced")
    }

    fn update(&mut self, message: Message) {
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
