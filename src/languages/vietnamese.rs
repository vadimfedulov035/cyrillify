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
            // LATIN LETTERS + "qu" DIGRAPH + đ/ư
            "a" => "а", "b" => "б", "c" => "к", "d" => "з",
            "e" => "е", "f" => "ф", "g" => "г", "h" => "х",
            "i" => "и", "j" => "ч", "k" => "к", "l" => "л",
            "m" => "м", "n" => "н", "o" => "о", "p" => "п",
            "r" => "р", "s" => "ш", "t" => "т", "u" => "у",
            "v" => "в", "w" => "в", "x" => "с", "y" => "и",
            "z" => "з", "qu" => "кв", "đ" => "д", "ư" => "ы",

            // Silent "h"
            "gh" => "г",
            "ph" => "ф",

            // Voiced "h"
            "th" => "тх",

            // T sound
            "ch" => "т",
            "ch|" => "ть",
            "cha" => "тя",
            "chi" => "ти",
            "chia" => "тья",
            "chie" => "тье",
            "chio" => "тё",
            "chiu" => "тью",
            "cho" => "тё",
            "chô" => "тё",
            "chơ" => "тё",
            "chu" => "тью",
            "chư" => "тьы",

            // Z sound
            "gi" => "з",
            "|gi|" => "зи",
            "giê" => "зье",
            "giư" => "зьы",
            "gia" => "зя",
            "gie" => "зе",
            "gio" => "зё",
            "giô" => "зё",
            "giơ" => "зё",
            "giu" => "зю",

            // N sound
            "ngh" => "нг",
            "nh" => "нь",
            "nha" => "ня",
            "nhe" => "не",
            "nhi" => "ни",
            "nho" => "нё",
            "nhô" => "нё",
            "nhơ" => "нё",
            "nhu" => "ню",

            // CH sound
            "tr" => "ч",

            // AY/EY
            "ây" => "ай",
            "tây" => "тэй",
            "plây" => "плей",

            // E
            "|e" => "э",
            "ae" => "аэ",
            "ie" => "иэ",
            "oe" => "оэ",
            "ue" => "уэ",
            "ưe" => "ыэ",

            // Ê
            "" => "",

            // IÊ
            "|iê" => "йе",
            "iê" => "ье",

            // UI/UY
            "ui" => "уй",    "uy" => "уи",

            "qui" => "куи",  "quy" => "куи",

            "hui" => "хюи",  "huy" => "хюи",
            "khui" => "кюи", "khuy" => "кюи",
            "thui" => "тюи", "thuy" => "тюи",

            "chui" => "тюй", "chuy" => "тюи",
            "nhui" => "нюй", "nhuy" => "нюи",
            "giui" => "зюй", "giuy" => "зюи",
            "lui" => "люй",  "luy" => "люи",
            "xui" => "сюй",  "xuy" => "сюи",

            // UE
            "xue" => "сюэ",

            // YÊ
            "yê" => "йе",

            // Y
            "|y" => "й",
            "ay" => "ай",

            "ey" => "ей",
            "aey" => "аэй",
            "eey" => "ээй",
            "iey" => "иэй",
            "oey" => "оэй",
            "uey" => "уэй",

            "iy" => "ий",
            "oy" => "ой",

            // UYÊ
            "uyê" => "уе",

            "huyê" => "хюе",
            "chuyê" => "тюе",
            "nhuyê" => "нюе",
            "giuyê" => "зюе",
            "luyê" => "люе",
            "xuyê" => "сюе",
        }
    }
}
