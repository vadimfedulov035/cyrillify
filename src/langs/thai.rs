use crate::transcriber::FormatRules;
use crate::transcriber::TranscriberTrait;

pub struct Transcriber {
    max_key_len: usize,
    format_rules: FormatRules,
}

// Thai transcriber constructor
impl std::default::Default for Transcriber {
    fn default() -> Self {
        Self {
            max_key_len: 3,
            format_rules: FormatRules {
                prefix_start: false,
                postfix_end: false,
            },
        }
    }
}

// Thai transcriber
impl TranscriberTrait for Transcriber {
    fn get_lang_name(&self) -> &'static str {
        "Тайский".into()
    }

    fn get_max_key_len(&self) -> usize {
        self.max_key_len
    }

    fn get_format_rules(&self) -> FormatRules {
        self.format_rules
    }

    fn get_transcription_str(&self, key: &str) -> Option<&'static str> {
        hashify::tiny_map! {
            key.as_bytes(),
            // LATIN LETTERS
            "A" => "А", "B" => "Б", "C" => "К", "D" => "Д",
            "E" => "Е", "F" => "Ф", "G" => "Г", "H" => "Х",
            "I" => "И", "J" => "Ч", "K" => "К", "L" => "Л",
            "M" => "М", "N" => "Н", "O" => "О", "P" => "П",
            "Q" => "КВ", "R" => "Р", "S" => "С", "T" => "Т",
            "U" => "У", "V" => "В", "W" => "В", "X" => "КС",
            "Y" => "Й", "Z" => "З",

            // A sound
            "AE" => "Э",
            "AEO" => "ЭУ",
            "AI" => "АЙ",
            "AO" => "АУ",

            // I sound
            "IA" => "ИА",
            "IAO" => "ИО",
            "IU" => "ИУ",

            // O sound
            "OE" => "Е",
            "OEI" => "ЕЙ",
            "OI" => "ОЙ",

            // U sound
            "UA" => "УА",
            "UAI" => "УАЙ",
            "UE" => "Ы",
            "UEA" => "ЫА",

            // CH sound
            "CH" => "Ч",
            "ÇH" => "Ч",
        }
    }
}
