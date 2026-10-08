use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::{Element, Fill};

fn main() -> iced::Result {
    iced::application(Messenger::default, Messenger::update, Messenger::view)
        .title("Fade Messenger")
        .run()
}

#[derive(Default)]
struct Messenger {
    draft: String,
    messages: Vec<String>,
}

#[derive(Debug, Clone)]
enum Message {
    DraftChanged(String),
    Send,
}

impl Messenger {
    fn update(&mut self, message: Message) {
        match message {
            Message::DraftChanged(draft) => self.draft = draft,
            Message::Send => {
                let draft = self.draft.trim();
                if !draft.is_empty() {
                    self.messages.push(draft.to_string());
                    self.draft.clear();
                }
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let history = self
            .messages
            .iter()
            .fold(column![].spacing(8), |col, msg| col.push(text(msg)));

        let input = row![
            text_input("Type a message...", &self.draft)
                .on_input(Message::DraftChanged)
                .on_submit(Message::Send)
                .padding(10),
            button("Send").on_press(Message::Send).padding(10),
        ]
        .spacing(8);

        container(
            column![scrollable(history).height(Fill), input]
                .spacing(12)
                .padding(16),
        )
        .into()
    }
}
