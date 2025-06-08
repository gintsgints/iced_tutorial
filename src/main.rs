use iced::{
    Element, Task as Command,
    widget::{button, column, row, text, text_input, text_input::Id},
};

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view).run()
}

/// Application state
#[derive(Default)]
struct App {
    name_field: String,
    name: String,
}

#[derive(Debug, Clone)]
enum Message {
    SayHello,
    UpdateInput(String),
}

use std::sync::LazyLock;
static TEXT_INPUT_ID: LazyLock<Id> = LazyLock::new(|| Id::new("name_input"));

impl App {
    fn new() -> (Self, Command<Message>) {
        (
            // Initialize the application state
            App {
                name: String::from("Iced"),
                ..Default::default()
            },
            // Focus the text input when the application starts
            text_input::focus(TEXT_INPUT_ID.clone()),
        )
    }

    // Update the application state here
    fn update(&mut self, message: Message) {
        match message {
            Message::UpdateInput(content) => {
                self.name_field = content;
            }
            Message::SayHello => {
                self.name = self.name_field.clone();
            }
        }
    }

    // Construct the view of the application based on the current state
    fn view(&self) -> Element<Message> {
        let button = button("Ok").on_press(Message::SayHello);
        let hello_text = text(format!("Hello, {}!", self.name));

        let name_input = text_input(
            "Enter your name, and press Enter or Ok button",
            &self.name_field,
        )
        .on_input(Message::UpdateInput)
        .on_submit(Message::SayHello)
        .id(TEXT_INPUT_ID.clone());
        let name_field = row![name_input, button];
        column![name_field, hello_text].into()
    }
}
