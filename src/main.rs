#![windows_subsystem = "windows"]

use iced::widget::{column, container, pick_list, text, text_input};
use iced::{Alignment, Element, Length, Task as Command, Theme};

mod languages;
mod transcriber;

const SUPPORTED_LANGUAGES: &[&str] = &["Тайский", "Бирманский"];

pub fn main() -> iced::Result {
    iced::application("Cyrillify", Cyrillify::update, Cyrillify::view)
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
        let max_len;
        let get_transcription: fn(&str, bool) -> Option<&'static str>;
        let mark_word_start;

        match self.selected_language {
            "Тайский" => {
                max_len = languages::thai::MAX_KEY_LEN;
                get_transcription = languages::thai::get_transcription;
                mark_word_start = false; // with |
            }
            "Бирманский" => {
                max_len = languages::burmese::MAX_KEY_LEN;
                get_transcription = languages::burmese::get_transcription;
                mark_word_start = true; // with |
            }
            _ => return,
        }

        self.output_text = transcriber::transcribe(
            &self.input_text,
            max_len,
            get_transcription,
            mark_word_start,
        );
    }
}

impl Cyrillify {
    fn new() -> (Self, Command<Message>) {
        (
            Self {
                selected_language: SUPPORTED_LANGUAGES[0],
                input_text: String::new(),
                output_text: String::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Cyrillify".into()
    }

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
            SUPPORTED_LANGUAGES.to_vec(),
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

    fn theme(&self) -> Theme {
        Theme::default()
    }
}
