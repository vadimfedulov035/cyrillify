pub const MAX_KEY_LEN: usize = 3;

fn get_transcription_str(key: &str) -> Option<&'static str> {
    hashify::tiny_map! {
        key.as_bytes(),
        // LATIN LETTERS + "qu" DIGRAPH
        "a" => "а", "b" => "б", "c" => "к", "d" => "д",
        "e" => "е", "f" => "ф", "g" => "г", "h" => "х",
        "i" => "и", "j" => "ч", "k" => "к", "l" => "л",
        "m" => "м", "n" => "н", "o" => "о", "p" => "п",
        "q" => "кв", "r" => "р", "s" => "с", "t" => "т",
        "u" => "у", "v" => "в", "w" => "в", "x" => "кс",
        "y" => "й", "z" => "з",  "qu" => "кв",

        // "a" patterns
        "ae" => "э",
        "aeo" => "эу",
        "ai" => "ай",
        "ao" => "ау",

        // "i" patterns
        "ia" => "иа",
        "iao" => "ио",
        "iu" => "иу",

        // "o" patterns
        "oe" => "е",
        "oei" => "ей",
        "oi" => "ой",

        // "u" patterns
        "ua" => "уа",
        "uai" => "уай",
        "ue" => "ы",
        "uea" => "ыа",

        // CH sound
        "ch" => "ч",
        "çh" => "ч",
    }
}

pub fn get_transcription(key: &str, _: bool) -> Option<&'static str> {
    get_transcription_str(key)
}
