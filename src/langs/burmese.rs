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
            max_key_len: 5,
            format_rules: FormatRules {
                prefix_start: true,
                postfix_end: false,
            },
        }
    }
}

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
            // All names are from wikipedia.org/wiki/name_name
            // ! means only transcription alignment
            // ? means deduction
            // ??? means generalization
            key.as_bytes(),
            // =================================================================
            //  LETTERS
            // =================================================================
            "A" => "А", "B" => "Б", "C" => "К", "D" => "Д", "E" => "Е",
            "F" => "Ф", "G" => "Г", "H" => "Х", "I" => "И", "J" => "ДЖ",
            "K" => "К", "L" => "Л", "M" => "М", "N" => "Н", "O" => "О",
            "P" => "П", "Q" => "К", "R" => "Р", "S" => "С", "T" => "Т",
            "U" => "У", "V" => "В", "W" => "В", "X" => "КС", "Y" => "Й",
            "Z" => "З",

            // =================================================================
            //  VOWELS & DIPHTONGS
            // =================================================================
            "AE" => "Э",    // Borrowed words ?
            "AI" => "АЙ",   // Min Aung Hlaing -> Мин Аун Хлайн (SAC chairman)
            "AW" => "О",    // Htin Kyaw -> Тхин Чо (9-th president) !
                            // Saw Maung -> Со Маун (1-st SPDC chairman)
            "AY" => "ЕЙ",   // Nay Pyi Taw -> Нейпидо (capital)
            "AYE" => "Э",   // Maung Aye -> Маун Э (2-nd SPDC vice chairman) !
                            // Though <Aye Ko -> Е Ко (vice president)>,
                            // to avoid confusion with "YE" (Е), "Э" is used
            "EE" => "И",    // Mee-Bone-Pyan U Kyaw Yin ->
                            // Ми-Бон-Пьян У Чжо Йин
            "EI" => "ЕЙ",   // Thein Sein -> Тейн Сейн (8-th president)
                            // Sein Lwin -> Сейн Лвин (6-th president)
            "OE" => "О",    // Soe Win -> Со Вин (10-th prime minister)
            "OO" => "У",    // Mya Tun Oo -> Мья Тун У (deputy prime minister)
            "ON" => "ОУН",  // Monywa -> Моунъюа (capital town)
            "YE" => "Е",    // Ye, Myanmar -> Е, Мьянма (town)
            "YA" => "Я",    // Yangon -> Янгон (city)
            "YW" => "Ю",    // Monywa -> Моунъюа (capital town)
            // Unvoiced vowels
            "ONE|" => "ОН",  // Mee-Bone-Pyan U Kyaw Yin ->
                             // Ми-Бон-Пьян У Чжо Йин
            "OKE|" => "ОУ",  // Hoke -> Хоу
            "INE|" => "АЙН", // Hain -> Хайн ? ... Only a suposal as -INE
                             // is rarely used in comparison to simpler -AING

            // =================================================================
            //  CONSONANT & ASPIRANTS
            // =================================================================
            // SH sound
            "SH" => "Ш",    // Daw Kin Win Shwe -> До Кин Вин Швэ
            "SW" => "ШВ",   // Myint Swe -> Мьин Шве
            // Aspiration consonants (inverse)
            "HK" => "КХ",   // Hkun Law -> Кхун Ло (king of Martaban) ?
            "HP" => "ПХ",   // Hpa-an -> Пхаан (capital town)
            "HT" => "ТХ",   // Htin Kyaw -> Тхин Чо (9-th president)
            // Unvoiced consonants
            "AR" => "А",   // Myanmar -> Мьянма (country)
            "AUK" => "АУ",  // Sai Mauk Kham -> Сайн Мау Кхан (vice president)
                            // Kyaukse -> Чжаусе (town) ?
            "AT" => "А",    // Kyat -> Чжа (currency)
            "ET" => "E",    // Htet Htet Moe Oo -> Тхе Тхе Мьо У (actress) ?
            "IT" => "И",    // Myitkyina -> Мьичина (capital town)
                            // Sittaung -> Ситаун (river)
                            // Though everything is correct here,
                            // <at/et/it> are reducted only at the syllable end,
                            // so syllable splitting is necessary later
            "INT|" => "ИН", // Myint Swe -> Мьин Шве (vice president)
            "NG|" => "Н",   // Thakin Kodaw Hmaing -> Такин Кодо Хмайн (poet)
            "NT|" => "Н",   // U Thant -> У Тан (3-rd UN secretary-general)
            "|TH" => "Т",   // Than Shwe -> Тан Шве (2-nd SPDC chairman)
            // Palatalized consontants
            "CH" => "Ч",    // Borrowed words ?
            "GY" => "ДЖ",   // Myo Gyi -> Мьо Джи (musician) ?
            "KY" => "Ч",    // Myitkyina -> Мьичина (capital town)
            "MY" => "МЬ",   // Myanmar -> Мьянма (country)
            "NY" => "НЬ",   // Paganyaw -> Паганья (ethnic group)
            "PY" => "ПЬ",   // Pyi -> Пьи (city)
            // Palatalized explosive consonant
            "|KY" => "ЧЖ",  // Kyaukse -> Чжаусе (town) ?
                            // Aung San Suu Kyi -> Аун Сан Су Чжи
                            // Htin Kyaw -> Тхин Чжо (9-th president)

            // =================================================================
            //  Palatalized consonants * YA/YW/INE
            // =================================================================
            "MYA" => "МЬЯ",     // Myanmar -> Мьянма (country)
            "NYA" => "НЬЯ",     // Nyaung-U -> Ньяун-У (town) ?
            "PYA" => "ПЬЯ",     // Bala Pyan -> Бала Пьян (dancer) ?

            "KYW" => "ЧЬЮ",     // ???
            "MYW" => "MЬЮ",     // ???
            "NYW" => "НЪЮ",     // Monywa -> Моунъюа (capital town)
            "PYW" => "ПЬЮ",     // Pyu -> Пью (city-states)
            // Conjunction of ON + NYW patterns
            "ONYW" => "ОУНЪЮ",  // Monywa -> Моунъюа (capital town)

            "KYINE|" => "ЧЬЯЙН", // ???
            "MYINE|" => "МЬЯЙН", // Mawlamyine (Mawlamyaing) -> Моламья́йн
            "NYINE|" => "НЬЯЙН", // ???
            "PYINE|" => "ПЬЯЙН", // ???

            // =================================================================
            //  Dictionary
            // =================================================================

            "|SAI|" => "САЙН", // Sai Mauk Kham -> Сайн Мау Кхан
        }
    }
}
