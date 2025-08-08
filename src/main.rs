#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use iced::widget::{column, container, pick_list, text, text_input};
use iced::{Alignment, Element, Length, Task as Command};
use strum::IntoEnumIterator;

mod casing;
mod langs;
mod transcriber;

#[cfg(test)]
mod tests;

use transcriber::TranscriberEnum;
use transcriber::TranscriberTrait;

pub fn main() -> iced::Result {
    iced::application("Cyrillify v0.4.0", Cyrillify::update, Cyrillify::view)
        .window_size((400.0, 420.0))
        .run()
}

#[derive(Default)]
struct Cyrillify {
    selected_language: &'static str,
    input_text: String,
    output_text: String,
}

#[derive(Debug, Clone)]
enum Message {
    LanguageChanged(&'static str),
    InputChanged(String),
    OutputEdited,
}

impl Cyrillify {
    fn retranscribe(&mut self) {
        let transcriber = TranscriberEnum::iter()
            .find(|transcriber| {
                transcriber.get_lang_name() == self.selected_language
            })
            .expect("Transcriber for selected language not found.");

        self.output_text = transcriber.transcribe(&self.input_text);
    }
}

impl Cyrillify {
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::LanguageChanged(language) => {
                self.selected_language = language;
                self.retranscribe();
            }
            Message::InputChanged(new_text) => {
                self.input_text = new_text;
                self.retranscribe();
            }
            Message::OutputEdited => {}
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let language_picker = pick_list(
            TranscriberEnum::iter()
                .map(|variant| variant.get_lang_name())
                .collect::<Vec<_>>(),
            Some(self.selected_language),
            Message::LanguageChanged,
        );

        let input_field = text_input("Введите имя", &self.input_text)
            .on_input(Message::InputChanged);

        let output_field = text_input("Транскрипция", &self.output_text)
            .on_input(|_| Message::OutputEdited);

        let content = column![
            text("Язык:"),
            language_picker,
            text("Имя:"),
            input_field,
            text("Транскрипция:"),
            output_field,
        ]
        .spacing(15)
        .align_x(Alignment::Center)
        .padding([0, 20]);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .into()
    }
}
