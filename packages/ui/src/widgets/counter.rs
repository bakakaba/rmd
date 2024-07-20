use iced::{
    widget::{button, column, text},
    Alignment, Element, Sandbox,
};

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

#[derive(Default)]
pub struct Counter {
    value: i64,
}

impl Counter {
    type Message = Message;

    fn new() -> Counter {
        Counter::default()
    }

    fn title(&self) -> String {
        String::from("A cool counter")
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

    fn view(&self) -> Element<Message> {
        // The buttons
        let increment = button("+").on_press(Message::Increment);
        let decrement = button("-").on_press(Message::Decrement);

        // The number
        let counter = text(self.value);

        column![increment, counter, decrement]
            .padding(20)
            .align_items(Alignment::Center)
            .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_counts_properly() {
        let mut counter = Counter { value: 0 };

        counter.update(Message::Increment);
        counter.update(Message::Increment);
        counter.update(Message::Decrement);

        assert_eq!(counter.value, 1);
    }
}
