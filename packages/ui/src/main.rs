mod widgets;

use iced::Settings;

fn main() {
    widgets::Counter::run(Settings::default())
}
