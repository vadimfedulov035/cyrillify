#![windows_subsystem = "windows"]

use iced::widget::{column, container, pick_list, text, text_input};
use iced::{
    Alignment, Application, Command, Element, Length, Settings, Size, Theme, executor, window,
};

mod handler;
mod languages;

use std::fmt;

pub fn main() -> iced::Result {
    let settings = Settings {
        window: window::Settings {
            size: Size::new(400.0, 420.0),
            ..window::Settings::default()
        },
        ..Settings::default()
    };
    CyrillifyApp::run(settings)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Language {
    name: &'static str,
    id: &'static str,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

const SUPPORTED_LANGUAGES: &[Language] = &[
    Language {
        name: "Thai",
        id: "thai",
    },
    Language {
        name: "Burmese",
        id: "burmese",
    },
];

struct CyrillifyApp {
    selected_language: Language,
    input_text: String,
    output_text: String,
}

#[derive(Debug, Clone)]
enum Message {
    LanguageChanged(Language),
    InputChanged(String),
    // "Dummy" message ignored by update
    OutputEdited(()),
}

// Helper function to keep transcription logic in one place
impl CyrillifyApp {
    fn re_cyrillify(&mut self) {
        if self.input_text.is_empty() {
            self.output_text.clear();
            return;
        }

        let result = match self.selected_language.id {
            "thai" => handler::cyrillify(
                &self.input_text,
                languages::thai::MAX_KEY_LEN,
                languages::thai::get_cyrillic,
            ),
            "burmese" => handler::cyrillify(
                &self.input_text,
                languages::burmese::MAX_KEY_LEN,
                languages::burmese::get_cyrillic,
            ),
            _ => "Language not implemented.".to_string(),
        };
        self.output_text = result;
    }
}

impl Application for CyrillifyApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
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
        String::from("Cyrillify")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::LanguageChanged(language) => {
                self.selected_language = language;
                self.re_cyrillify();
            }
            Message::InputChanged(new_text) => {
                self.input_text = new_text;
                self.re_cyrillify();
            }
            // Do nothing if output edited to prevent changing text
            Message::OutputEdited(_) => {}
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let language_picker = pick_list(
            &SUPPORTED_LANGUAGES[..],
            Some(self.selected_language),
            Message::LanguageChanged,
        );

        let input_field = text_input("Enter or paste Roman letters...", &self.input_text)
            .on_input(Message::InputChanged)
            .padding(10);

        // Output field is text_input widget for selection/copying.
        // The `on_input(Message::OutputEdited)` makes it "active", but update prevents changes
        let output_field = text_input("Result appears here...", &self.output_text)
            .on_input(|_: String| Message::OutputEdited(()))
            .padding(10);

        let content = column![
            text("Language:"),
            language_picker,
            text("Input:"),
            input_field,
            text("Output:"),
            output_field,
        ]
        .spacing(15)
        .align_items(Alignment::Center)
        .padding(20);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::default()
    }
}
