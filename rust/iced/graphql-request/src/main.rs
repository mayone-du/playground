// use graphql_client::{GraphQLQuery, Response};
use iced::{
    button, executor, scrollable, Application, Button, Clipboard, Column, Command, Element,
    Scrollable, Settings, Text,
};
use reqwest;

pub fn main() -> iced::Result {
    State::run(Settings::default())
}

#[derive(Debug)]
enum RootState {
    Loading,
    Loaded(State),
    Error,
}

#[derive(Default, Debug)]
struct AppState {
    scroll: scrollable::State,
    value: String,
    request_button: button::State,
}

#[derive(Default, Debug)]
struct State {
    name: String,
    description: String,
    age: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    RequestPressed,
    RequestResult(Result<(), ()>),
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
                // Command::perform(SampleRequest::sample_request(), |data| {
                //     println!("{:?}", data);
                // });
                Command::perform(SampleRequest::sample_request(), Message::RequestResult)

                // let result = reqwest::blocking::get("https://jsonplaceholder.typicode.com/posts/");
                // let result = match result {
                //     Ok(response) => response.text(),
                //     Err(e) => {
                //         panic!("{:?}", e);
                //     }
                // };
                // self.value = result.unwrap().to_string();
            }
            Message::RequestResult(result) => match result {
                Ok(response) => {
                    println!("{:?}", response);
                }
                Err(e) => {
                    panic!("{:?}", e);
                }
            },
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

#[derive(Debug, Clone)]
struct SampleRequest {
    value: String,
}

impl SampleRequest {
    async fn sample_request() -> String {
        let client = reqwest::Client::new();
        let text = client
            .get("https://jsonplaceholder.typicode.com/todos/1")
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
            .to_string();
        println!("{:?}", text);
        text
    }
}
