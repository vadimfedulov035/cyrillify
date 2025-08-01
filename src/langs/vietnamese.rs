use crate::transcriber::FormatRules;
use crate::transcriber::TranscriberTrait;

pub struct Transcriber {
    max_key_len: usize,
    format_rules: FormatRules,
}

// Vietnamese constructor
impl std::default::Default for Transcriber {
    fn default() -> Self {
        Self {
            max_key_len: 4,
            format_rules: FormatRules {
                prefix_start: true,
                postfix_end: true,
            },
        }
    }
}

// Vietnamese transcriber
impl TranscriberTrait for Transcriber {
    fn get_lang_name(&self) -> &'static str {
        "Вьетнамский".into()
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
            // LATIN LETTERS + đ/ư
            "A" => "А", "B" => "Б", "C" => "К", "D" => "З",
            "E" => "Е", "F" => "Ф", "G" => "Г", "H" => "Х",
            "I" => "И", "J" => "Ч", "K" => "К", "L" => "Л",
            "M" => "М", "N" => "Н", "O" => "О", "P" => "П",
            "R" => "Р", "S" => "Ш", "T" => "Т", "U" => "У",
            "V" => "В", "W" => "В", "X" => "С", "Y" => "И",
            "Z" => "З", "Đ" => "Д", "Ư" => "Ы",

            // Silent "h"
            "GH" => "Г",
            "PH" => "Ф",

            // Voiced "h"
            "TH" => "ТХ",

            // T sound
            "CH" => "Т",
            "CH|" => "ТЬ",
            "CHA" => "ТЯ",
            "CHI" => "ТИ",
            "CHIA" => "ТЬЯ",
            "CHIE" => "ТЬЕ",
            "CHIO" => "ТЁ",
            "CHIU" => "ТЬЮ",
            "CHO" => "ТЁ",
            "CHÔ" => "ТЁ",
            "CHƠ" => "ТЁ",
            "CHU" => "ТЬЮ",
            "CHƯ" => "ТЬЫ",

            // Z sound
            "GI" => "З",
            "|GI|" => "ЗИ",
            "GIÊ" => "ЗЬЕ",
            "GIƯ" => "ЗЬЫ",
            "GIA" => "ЗЯ",
            "GIE" => "ЗЕ",
            "GIO" => "ЗЁ",
            "GIÔ" => "ЗЁ",
            "GIƠ" => "ЗЁ",
            "GIU" => "ЗЮ",

            // N sound
            "NGH" => "НГ",
            "NH" => "НЬ",
            "NHA" => "НЯ",
            "NHE" => "НЕ",
            "NHI" => "НИ",
            "NHO" => "НЁ",
            "NHÔ" => "НЁ",
            "NHƠ" => "НЁ",
            "NHU" => "НЮ",

            // CH sound
            "TR" => "Ч",

            // AY/EY
            "ÂY" => "АЙ",
            "TÂY" => "ТЭЙ",
            "PLÂY" => "ПЛЕЙ",

            // E
            "|E" => "Э",
            "AE" => "АЭ",
            "IE" => "ИЭ",
            "OE" => "ОЭ",
            "UE" => "УЭ",
            "ƯE" => "ЫЭ",

            // Ê
            // "" => "",

            // IÊ
            "|IÊ" => "ЙЕ",
            "IÊ" => "ЬЕ",

            // UI/UY
            "UI" => "УЙ",    "UY" => "УИ",

            "QUI" => "КУИ",  "QUY" => "КУИ",

            "HUI" => "ХЮИ",  "HUY" => "ХЮИ",
            "KHUI" => "КЮИ", "KHUY" => "КЮИ",
            "THUI" => "ТЮИ", "THUY" => "ТЮИ",

            "CHUI" => "ТЮЙ", "CHUY" => "ТЮИ",
            "NHUI" => "НЮЙ", "NHUY" => "НЮИ",
            "GIUI" => "ЗЮЙ", "GIUY" => "ЗЮИ",
            "LUI" => "ЛЮЙ",  "LUY" => "ЛЮИ",
            "XUI" => "СЮЙ",  "XUY" => "СЮИ",

            // UE
            "XUE" => "СЮЭ",

            // YÊ
            "YÊ" => "ЙЕ",

            // Y
            "|Y" => "Й",
            "AY" => "АЙ",

            "EY" => "ЕЙ",
            "AEY" => "АЭЙ",
            "EEY" => "ЭЭЙ",
            "IEY" => "ИЭЙ",
            "OEY" => "ОЭЙ",
            "UEY" => "УЭЙ",

            "IY" => "ИЙ",
            "OY" => "ОЙ",

            // UYÊ
            "UYÊ" => "УЕ",

            "HUYÊ" => "ХЮЕ",
            "CHUYÊ" => "ТЮЕ",
            "NHUYÊ" => "НЮЕ",
            "GIUYÊ" => "ЗЮЕ",
            "LUYÊ" => "ЛЮЕ",
            "XUYÊ" => "СЮЕ",
        }
    }
}
