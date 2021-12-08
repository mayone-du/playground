use iced::{
    button, executor, scrollable, Application, Button, Clipboard, Column, Command, Element,
    Scrollable, Settings, Text,
};
use iced_winit::{event, winit, winit::event_loop::EventLoop};
use reqwest;

pub fn main() -> iced::Result {
    // let event_loop = EventLoop::new();
    // let window = winit::window::Window::new(&event_loop).unwrap();
    State::run(Settings::default())
}

#[derive(Default, Debug)]
struct State {
    scroll: scrollable::State,
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

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::RequestPressed => {
                let result = reqwest::blocking::get("https://jsonplaceholder.typicode.com/posts/");
                let result = match result {
                    Ok(response) => response.text(),
                    Err(e) => {
                        panic!("{:?}", e);
                    }
                };
                self.value = result.unwrap().to_string();
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let text = Text::new(&self.value).size(10);
        let request_button = Button::new(&mut self.request_button, Text::new("Request"))
            .on_press(Message::RequestPressed);
        let content = Column::new()
            .padding(20)
            .spacing(20)
            .max_width(500)
            .push(text)
            .push(request_button);

        Scrollable::new(&mut self.scroll).push(content).into()
    }
}
