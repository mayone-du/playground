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
    number_buttons: [button::State; 10],
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
                Operation::Subtract => {
                    println!("subtract");
                }
                Operation::Multiply => {
                    println!("multiply");
                }
                Operation::Divide => {
                    println!("divide");
                }
                Operation::Reset => {
                    println!("reset");
                }
                Operation::Calculate => {
                    println!("calculate");
                }
                _ => {
                    println!("other");
                }
            },
            _ => {}
        }
    }

    fn view(&mut self) -> Element<Message> {
        // add
        let add_button = Button::new(
            &mut self.add_operation_button,
            Text::new("add").color(Color::from_rgb(0.0, 0.0, 0.0)),
        )
        .on_press(Message::ButtonPressed(Operation::Add));
        // subtract
        let subtract_button = Button::new(
            &mut self.subtract_operation_button,
            Text::new("subtract").color(Color::from_rgb(0.0, 0.0, 0.0)),
        )
        .on_press(Message::ButtonPressed(Operation::Subtract));
        // multiply
        let multiply_button = Button::new(
            &mut self.multiply_operation_button,
            Text::new("multiply").color(Color::from_rgb(0.0, 0.0, 0.0)),
        )
        .on_press(Message::ButtonPressed(Operation::Multiply));
        // divide
        let divide_button = Button::new(
            &mut self.divide_operation_button,
            Text::new("divide").color(Color::from_rgb(0.0, 0.0, 0.0)),
        )
        .on_press(Message::ButtonPressed(Operation::Multiply));

        // number_buttons
        let number_buttons = vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
            .iter()
            .map(|i| {
                Button::new(
                    &mut self.number_buttons[i as u32],
                    Text::new(i.to_string()).color(Color::from_rgb(0.0, 0.0, 0.0)),
                )
                .on_press(Message::ButtonPressed(Operation::Reset))
            })
            .collect::<Vec<_>>();

        // カラム
        Column::new()
            .padding(20)
            // ボタンを生成し、クリックイベントにMessageのIncrementPressedを指定
            .push(add_button)
            .push(subtract_button)
            .push(multiply_button)
            .push(divide_button)
            .into()
    }
}
