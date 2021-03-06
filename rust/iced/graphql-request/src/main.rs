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
    RequestResult(Result<String, AppError>),
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
                    // ! Loadingが表示されるのを確認するために3秒待機3秒待機
                    let time = std::time::Duration::from_secs(3);
                    std::thread::sleep(time);

                    // &mutなselfの参照を外して、Loadedに変更する
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
    async fn sample_request() -> Result<String, AppError> {
        let client = reqwest::Client::new();
        let id = 1;
        let url = format!("https://jsonplaceholder.typicode.com/todos/{}", id.to_string());
        let text = client
            .get(url)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
            .to_string();
        Ok(text)
    }
}

#[derive(Debug, Clone)]
struct AppError {}