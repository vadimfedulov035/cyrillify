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
            // LATIN LETTERS + "qu" DIGRAPH
            "a" => "а", "b" => "б", "c" => "к", "d" => "д",
            "e" => "е", "f" => "ф", "g" => "г", "h" => "х",
            "i" => "и", "j" => "ч", "k" => "к", "l" => "л",
            "m" => "м", "n" => "н", "o" => "о", "p" => "п",
            "q" => "кв", "r" => "р", "s" => "с", "t" => "т",
            "u" => "у", "v" => "в", "w" => "в", "x" => "кс",
            "y" => "й", "z" => "з", "qu" => "кв",

            // A sound
            "ae" => "э",
            "aeo" => "эу",
            "ai" => "ай",
            "ao" => "ау",

            // I sound
            "ia" => "иа",
            "iao" => "ио",
            "iu" => "иу",

            // O sound
            "oe" => "е",
            "oei" => "ей",
            "oi" => "ой",

            // U sound
            "ua" => "уа",
            "uai" => "уай",
            "ue" => "ы",
            "uea" => "ыа",

            // CH sound
            "ch" => "ч",
            "çh" => "ч",
        }
    }
}
