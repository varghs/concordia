use iced::widget::{button, column, text};
use iced::{Element, Sandbox, Settings};

mod library;
mod pdf;

/*
#[derive(Default)]
struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Counter {
        Self { value: 0 }
    }

    fn title(&self) -> String {
        String::from("Counter app")
    }

    fn view(&self) -> Element<Self::Message> {
        column![
            button("+").on_press(Message::Increment),
            text(self.value).size(50),
            button("-").on_press(Message::Decrement)
        ]
        .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }
}
*/

/*
fn main() -> iced::Result {
    // Counter::run(Settings::default())
}
*/

fn main() {}
