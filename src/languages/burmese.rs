use crate::transcriber::FormatRules;
use crate::transcriber::TranscriberTrait;

pub struct Transcriber {
    max_key_len: usize,
    format_rules: FormatRules,
}

// Burmese transcriber constructor
impl std::default::Default for Transcriber {
    fn default() -> Self {
        Self {
            max_key_len: 4,
            format_rules: FormatRules {
                prefix_start: true,
                postfix_end: false,
            },
        }
    }
}

/// Transcribes via MLCTS (Myanma Language Commission Transcription System)
// Burmese transcriber
impl TranscriberTrait for Transcriber {
    fn get_lang_name(&self) -> &'static str {
        "Бирманский".into()
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
            "a" => "а", "b" => "б", "c" => "с", "d" => "д",
            "e" => "э", "f" => "ф", "g" => "г", "h" => "х",
            "i" => "и", "j" => "з", "k" => "к", "l" => "л",
            "m" => "м", "n" => "н", "o" => "о", "p" => "п",
            "q" => "к", "r" => "ь", "s" => "т", "t" => "т",
            "u" => "у", "v" => "в", "w" => "в", "x" => "кс",
            "y" => "ь", "z" => "з", "qu" => "кв",

            // Silent "h"
            "bha" => "ба",
            "bhwa" => "бва",
            "dha" => "да",
            "gha" => "га",
            "jha" => "за",

            // Inversed "h"
            "hc" => "сх",
            "hk" => "кх",
            "hp" => "пх",
            "ht" => "тх",

            // SH sound
            "hra" => "ша",
            "hya" => "ша",
            "hlya" => "ша",
            "hsya" => "ша",
            "hrwa" => "швэй",

            // CH sound
            "kr" => "ч", "hkr" => "чх",
            "ky" => "ч", "hky" => "чх",

            // YA/Y sound
            "|r" => "я",
            "|y" => "я",
            "|ra" => "я",
            "|ya" => "я",

            "ay" => "ай", "aya" => "ая",
            "iy" => "ий", "iya" => "ия",
            "uy" => "уй", "uya" => "уя",
            "ey" => "эй", "eya" => "эя",
            "oy" => "ой", "oya" => "оя",

            "ar" => "ай", "ara" => "ая",
            "ir" => "ий", "ira" => "ия",
            "ur" => "уй", "ura" => "уя",
            "er" => "эй", "era" => "эя",
            "or" => "ой", "ora" => "оя",

            // Y sound
            "ywa" => "йва", "nywa" => "ньу",
            "rwa" => "йва", "prwa" => "пьу",

            // Irregular
            "gya" => "джа",
            "tra" => "тра",
            "ngra" => "ньа",
            "mrwa" => "мьва",

            // Finals
            "ac" => "и",

            "ai" => "э", "ak" => "э",
            "au" => "о", "auk" => "ау",
            "ui" => "оу", "uik" => "ай",

            "ang" => "ин", "any" => "и(н)",
            "aung" => "аун",
            "uing" => "айн",

            "ap" => "а", "at" => "а",
            "ip" => "эй", "it" => "эй",
            "up" => "оу", "ut" => "оу",

            "am" => "ан", "an" => "ан",
            "im" => "эйн", "in" => "эйн",
            "um" => "оун", "un" => "оун",
        }
    }
}
