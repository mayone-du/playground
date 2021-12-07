use iced::{
    button, executor, Application, Button, Clipboard, Column, Command, Element, Settings, Text,
};
use iced_winit::{event, winit, winit::event_loop::EventLoop};
use reqwest;

pub fn main() -> iced::Result {
    // let event_loop = EventLoop::new();
    // let window = winit::window::Window::new(&event_loop).unwrap();
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

    async fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::RequestPressed => {
                // let result = reqwest::blocking::get("https://jsonplaceholder.typicode.com/posts/");
                let result = reqwest::get("https://jsonplaceholder.typicode.com/posts/").await;
                match result {
                    Ok(response) => response.text().await,
                    Err(e) => {
                        panic!("{:?}", e);
                    }
                };
                match result {
                    Ok(response) => {
                        self.value = response.text().await.unwrap();
                    }
                    Err(e) => {
                        panic!("{:?}", e);
                    }
                }
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .push(Text::new(self.value.to_string()).size(10))
            .push(
                Button::new(&mut self.request_button, Text::new("Request"))
                    .on_press(Message::RequestPressed),
            )
            .into()
    }
}
