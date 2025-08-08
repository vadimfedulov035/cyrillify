#[cfg(test)]
use crate::tests::Mapping;

crate::create_transcriber!(
    lang_name: "Бирманский",
    lang_rules: {
        // All names are from wikipedia.org/wiki/name_name
        // ===================================================================
        //  LETTERS
        // ===================================================================
        "A" => "А", "B" => "Б", "C" => "К", "D" => "Д", "E" => "Е",
        "F" => "Ф", "G" => "Г", "H" => "Х", "I" => "И", "J" => "ДЖ",
        "K" => "К", "L" => "Л", "M" => "М", "N" => "Н", "O" => "О",
        "P" => "П", "Q" => "К", "R" => "Р", "S" => "С", "T" => "Т",
        "U" => "У", "V" => "В", "W" => "В", "X" => "КС", "Y" => "Й",
        "Z" => "З",

        // ===================================================================
        //  VOWELS & DIPHTONGS
        // ===================================================================
        // Voiced vowels
        "AE" => "Э",     // Borrowed words
        "AI" => "АЙ",    // Min Aung Hlaing -> Мин Аун Хлайн (SAC chairman)
        "AW" => "О",     // Htin Kyaw -> Тхин Чжо (president)
                         // Saw Maung -> Со Маун (SPDC chairman)
        "AY" => "ЕЙ",    // Nay Pyi Taw -> Нейпидо (capital)
        "AYE" => "Э",    // Maung Aye -> Маун Э (SPDC vice chairman)
                         //       Aye Ko -> Е Ко (vice president) <imprecize>
                         // "Э" suits better to avoid confusion with "YE" (Е)
        "EE" => "И",     // Mee-Bone-Pyan U Kyaw Yin ->
                         // Ми-Бон-Пьян У Чжо Йин
        "EI" => "ЕЙ",    // Thein Sein -> Тейн Сейн (president)
                         // Sein Lwin -> Сейн Лвин (president)
        "OE" => "О",     // Soe Win -> Со Вин (prime minister)
        "OO" => "У",     // Mya Tun Oo -> Мья Тун У (deputy prime minister)
        "ON" => "ОУН",   // Monywa -> Моунъюа (capital town)
        "YA" => "Я",     // Yangon -> Янгон (city)
        "YE" => "Е",     // Ye, Myanmar -> Е, Мьянма (town)
        "YU" => "Ю",     // San Yu -> Сан Ю (president)
        "YW" => "Ю",     // Monywa -> Моунъюа (capital town)
        // Unvoiced vowels
        "ONE|" => "ОН",  // Mee-Bone-Pyan U Kyaw Yin -> Ми-Бон-Пьян У Чжо Йин
        "OKE|" => "ОУ",  // Hoke -> Хоу
        "INE|" => "АЙН", // Hain -> Хайн, -INE is less common that -AING

        // ===================================================================
        //  CONSONANT & ASPIRANTS
        // ===================================================================
        // SH sound
        "SH" => "Ш",    // Daw Kin Win Shwe -> До Кин Вин Шве
        "SW" => "ШВ",   // Myint Swe -> Мьин Шве (vice president)
        // Aspiration consonants
        "HK" => "КХ",   // Hkun Law -> Кхун Ло (king of Martaban)
        "HP" => "ПХ",   // Hpa-an -> Пхаан (capital town)
        "HT" => "ТХ",   // Htin Kyaw -> Тхин Чжо (president)
        // Unvoiced consonants
        "AR" => "А",    // Myanmar -> Мьянма (country)
        "AUK" => "АУ",  // Sai Mauk Kham -> Сайн Мау Кхан (vice president)
                        // Kyaukse -> Чжаусе (town)
        "AT" => "А",    // Kyat -> Чжа (currency)
        "IT" => "И",    // Myitkyina -> Мьичина (capital town)
                        // Sittaung -> Ситаун (river)
        "|TH" => "Т",   // Than Shwe -> Тан Шве (prime minister)
        "INT|" => "ИН", // Myint Swe -> Мьин Шве (vice president)
        "NG|" => "Н",   // Thakin Kodaw Hmaing -> Такин Кодо Хмайн (poet)
        "NT|" => "Н",   // U Thant -> У Тан (UN secretary-general)
        "AIK|" => "АЙ", // Sao Shwe Thaik -> Сао Шве Тай (president)
        // Palatalized consontants
        "CH" => "Ч",    // Borrowed words
        "GY" => "ДЖ",   // Myo Gyi -> Мьо Джи (musician)
        "KY" => "Ч",    // Myitkyina -> Мьичина (capital town)
        "MY" => "МЬ",   // Myanmar -> Мьянма (country)
        "NY" => "НЬ",   // Paganyaw -> Паганья (ethnic group)
        "PY" => "ПЬ",   // Pyi -> Пьи (city)
        // Palatalized explosive consonants (position dependent)
        "|KY" => "ЧЖ",  // Htin Kyaw -> Тхин Чжо (president)
                        // Aung San Suu Kyi -> Аун Сан Су Чжи
                        // Kyaukse -> Чжаусе (town)

        // ===================================================================
        //  Palatalized consonants * (YA|YU/YO/YW|INE)
        // ===================================================================
        // "KYA" would be "ЧЖА", generalizable with KY+A -> ЧЖ+А
        "MYA" => "МЬЯ", // Myanmar -> Мьянма (country)
        "NYA" => "НЬЯ", // Nyaung-U -> Ньяун-У (town)
        "PYA" => "ПЬЯ", // Bala Pyan -> Бала Пьян (dancer)

        // YU/YO/YW patterns
        "KYU" => "ЧЬЮ", "KYO" => "ЧЬЮ", "KYW" => "ЧЬЮ",
        "MYU" => "МЬЮ", "MYO" => "МЬЮ", "MYW" => "MЬЮ",
        "NYU" => "НЬЮ", "NYO" => "НЬЮ", "NYW" => "НЬЮ",
        "PYU" => "ПЬЮ", "PYO" => "ПЬЮ", "PYW" => "ПЬЮ",
        // + ON pattern
        "ONYU" => "ОУНЬЮ", "ONYO" => "ОУНЬЮ", "ONYW" => "ОУНЬЮ",

        // YINE patterns
        "KYINE|" => "ЧЬЯЙН",
        "MYINE|" => "МЬЯЙН",
        "NYINE|" => "НЬЯЙН",
        "PYINE|" => "ПЬЯЙН",

        // ===================================================================
        //  Dictionary
        // ===================================================================

        "|SAI|" => "САЙН",   // Sai Mauk Kham -> Сайн Мау Кхан
        "|HTET|" => "ТХE",   // Htet Htet Moe Oo -> Тхе Тхе Мьо У (actress)

    },
    mappings: {
        vec![
            // Presidents of Mianmar
            Mapping { from: "Sao Shwe Thaik", to: "Сао Шве Тай" },
            Mapping { from: "Ba U", to: "Ба У" },
            Mapping { from: "Win Maung", to: "Вин Маун" },
            Mapping { from: "Ne Win", to: "Не Вин" },
            Mapping { from: "San Yu", to: "Сан Ю" },
            Mapping { from: "Sein Lwin", to: "Сейн Лвин" },
            Mapping { from: "Maung Maung", to: "Маун Маун" },
            Mapping { from: "Thein Sein", to: "Тейн Сейн" },
            Mapping { from: "Htin Kyaw", to: "Тхин Чжо" },

            // Prime ministers of Myanmar
            Mapping { from: "U Nu", to: "У Ну" },
            // Mapping { from: "Ba Swe", to: "Ба Све" }, // Exc.
            Mapping { from: "Ne Win", to: "Не Вин" },
            Mapping { from: "Sein Win", to: "Сейн Вин" },
            Mapping { from: "Maung Maung Kha", to: "Маун Маун Кха" },
            Mapping { from: "Tun Tin", to: "Тун Тин" },
            Mapping { from: "Saw Maung", to: "Со Маун" },
            Mapping { from: "Than Shwe", to: "Тан Шве" },
            Mapping { from: "Khin Nyunt", to: "Кхин Ньюн" },
            Mapping { from: "Soe Win", to: "Со Вин" },
            Mapping { from: "Thein Sein", to: "Тейн Сейн" },
            Mapping { from: "Min Aung Hlaing", to: "Мин Аун Хлайн" },
            Mapping { from: "Nyo Saw", to: "Нью Со" },

            // Deputy prime ministers
            Mapping { from: "Bo Let Ya", to: "Бо Лет Я" },
            Mapping { from: "Kyaw Nyein", to: "Чжо Ньейн" },
            Mapping { from: "Ne Win", to: "Не Вин" },
            Mapping { from: "Sao Hkun Hkio", to: "Сао Кхун Кхио" },
            Mapping { from: "Thein Maung", to: "Тейн Маун" },
            Mapping { from: "Lun Baw", to: "Лун Бо" },
            Mapping { from: "U Lwin", to: "У Лвин" },
            Mapping { from: "Tun Tin", to: "Тун Тин" },
            Mapping { from: "Thura Kyaw Htin", to: "Тура Чжо Тхин" },
            Mapping { from: "Than Shwe", to: "Тан Шве" },
            Mapping { from: "Khin Maung Yin", to: "Кхин Маун Йин" },
            Mapping { from: "Maung Maung Khin", to: "Маун Маун Кхин" },
            Mapping { from: "Tun Tin", to: "Тун Тин" },
            Mapping { from: "Tin Hla", to: "Тин Хла" },
            Mapping { from: "Soe Win", to: "Со Вин" },
            Mapping { from: "Mya Tun Oo", to: "Мья Тун У" },
            Mapping { from: "Tin Aung San", to: "Тин Аун Сан" },
            // Mapping { from: "Soe Htut", to: "Со Хтут" }, // Exc.
            Mapping { from: "Win Shein", to: "Вин Шейн" },
            Mapping { from: "Than Swe", to: "Тан Шве" },
            Mapping { from: "Maung Maung Aye", to: "Маун Маун Э" },

            // Vice presidents
            Mapping { from: "Tin Aung Myint Oo", to: "Тин Аун Мьин У" },
            Mapping { from: "Sai Mauk Kham", to: "Сайн Мау Кхам" },
            Mapping { from: "Nyan Tun", to: "Ньян Тун" },
            Mapping { from: "Myint Swe", to: "Мьин Шве" },
            // Mapping { from: "Henry Van Thio", to: "Генри Ван Тио" }, // Exc.

            // SPDC chairmen
            Mapping { from: "Saw Maung", to: "Со Маун" },
            Mapping { from: "Maung Aye", to: "Маун Э" },

            // Kings
            Mapping { from: "Hkun Law", to: "Кхун Ло" },

            // Toponyms
            Mapping { from: "Myanmar", to: "Мьянма" },

            // Names
            Mapping {
                from: "Mee-Bone-Pyan U Kyaw Yin", to: "Ми-Бон-Пьян У Чжо Йин",
            },
            Mapping { from: "Daw Kin Win Shwe", to: "До Кин Вин Шве" },
            Mapping { from: "Hain", to: "Хайн" },
            Mapping { from: "Hoke", to: "Хоу" },
        ]
    }
);
