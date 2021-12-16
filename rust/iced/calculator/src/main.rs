use iced::{
    button, text_input, Button, Color, Column, Command, Element, Sandbox, Settings, Text, TextInput,
};

pub fn main() -> iced::Result {
    State::run(Settings::default())
}

// アプリケーションのstate
#[derive(Debug, Default)]
struct State {
    previous_value: i32,
    current_value: i32,
    calclated_value: i32,
    add_operation_button: button::State,
    subtract_operation_button: button::State,
    multiply_operation_button: button::State,
    divide_operation_button: button::State,
    reset_operation_button: button::State,
    calculate_operation_button: button::State,
}

// 操作
#[derive(Debug, Clone)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Reset,
    Calculate,
}

// メッセージ
#[derive(Debug, Clone)]
enum Message {
    ButtonPressed(Operation),
    CurrentInputChanged(i32),
}

impl Sandbox for State {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Calculator")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ButtonPressed(operation) => match operation {
                Operation::Add => {
                    println!("add");
                }
                _ => {
                    println!("other");
                }
            },
            _ => {}
        }
    }

    fn view(&mut self) -> Element<Message> {
        // カラム
        Column::new()
            .padding(20)
            // ボタンを生成し、クリックイベントにMessageのIncrementPressedを指定
            .push(
                Button::new(
                    &mut self.add_operation_button,
                    Text::new("add").color(Color::from_rgb(100.0, 100.0, 0.0)),
                )
                .on_press(Message::ButtonPressed(Operation::Add)),
            )
            .push(
                Button::new(
                    &mut self.subtract_operation_button,
                    Text::new("subtract").color(Color::from_rgb(100.0, 100.0, 0.0)),
                )
                .on_press(Message::ButtonPressed(Operation::Subtract)),
            )
            .into()
    }
}
