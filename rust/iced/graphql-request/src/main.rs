// use graphql_client::{GraphQLQuery, Response};
use iced::{
    button, executor, scrollable, Application, Button, Clipboard, Column, Command, Container,
    Element, Scrollable, Settings, Text,
};
use reqwest;

pub fn main() -> iced::Result {
    GraphQLRequest::run(Settings::default())
}

#[derive(Debug)]
enum GraphQLRequest {
    Loading,
    Loaded {
        response: Response,
        button: button::State,
    },
    Errored,
}

#[derive(Debug)]
struct Response {
    data: String,
}

// #[derive(Default, Debug)]
// struct State {
//     scroll: scrollable::State,
//     value: String,
//     request_button: button::State,
// }

#[derive(Debug, Clone)]
enum Message {
    RequestPressed,
    RequestResult(Result<String, ()>),
}

impl Application for GraphQLRequest {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (GraphQLRequest, Command<Message>) {
        (
            GraphQLRequest::Loading,
            // SampleRequest::sample_request()で受け取ったOk(String)をMessage::RequestResultにわたしてる？
            Command::perform(SampleRequest::sample_request(), Message::RequestResult),
            // Command::none(), // 起動時には何もしない
        )
    }

    fn title(&self) -> String {
        let subtitle = match self {
            GraphQLRequest::Loading => "Loading...",
            GraphQLRequest::Loaded { .. } => "GraphQL Request",
            GraphQLRequest::Errored => "Errored!",
        };
        format!("Iced - {}", subtitle)
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::RequestPressed => {
                Command::perform(SampleRequest::sample_request(), Message::RequestResult);
            }
            Message::RequestResult(result) => match result {
                Ok(response) => {
                    *self = GraphQLRequest::Loaded {
                        response: Response { data: response },
                        button: button::State::new(),
                    };
                }
                Err(e) => {
                    *self = GraphQLRequest::Errored;
                    panic!("{:?}", e);
                }
            },
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let content = match self {
            GraphQLRequest::Loading => Column::new().push(Text::new("Loading...")),
            GraphQLRequest::Loaded { response, button } => {
                Column::new()
                    .padding(20)
                    .push(Text::new(&response.data))
                    .push(
                        Button::new(button, Text::new("Request")).on_press(Message::RequestPressed),
                    )
                // Text::new(&response.data);
            }
            GraphQLRequest::Errored => Column::new().push(Text::new("Errored!")),
        };

        Column::new().padding(20).push(content).into()
    }
}

#[derive(Debug, Clone)]
struct SampleRequest {
    value: String,
}

impl SampleRequest {
    async fn sample_request() -> Result<String, ()> {
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
        Ok(text)
    }
}
