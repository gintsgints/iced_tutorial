use iced::{
    Element,
    widget::{button, column, row, text, text_input},
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

impl App {
    fn new() -> Self {
        App {
            name: String::from("Iced"),
            ..Default::default()
        }
    }

    /// Update the application state here
    fn update(&mut self, message: Message) {
        match message {
            Message::SayHello => {
                self.name = self.name_field.clone();
            }
            Message::UpdateInput(content) => {
                self.name_field = content;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let button = button("Ok").on_press(Message::SayHello);
        let hello_text = text(format!("Hello, {}!", self.name));

        let name_input = text_input("Enter your name, and press Enter or Ok button", &self.name_field)
            .on_input(Message::UpdateInput)
            .on_submit(Message::SayHello);
        let name_field = row![name_input, button];
        column![name_field, hello_text].into()
    }
}
