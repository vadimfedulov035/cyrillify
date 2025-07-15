pub const MAX_KEY_LEN: usize = 3;

static CYRILLIC_LETTERS: &'static [&'static str] = &[
    "а", "б", "к", "д", "е", "ф", "г", "х", "и", "к", "ч", "л", "м", "н", "о", "п", "кв", "р", "с",
    "т", "у", "в", "в", "кс", "й", "з", "кв", "э", "эу", "ай", "ау", "иа", "ио", "иу", "е", "ей",
    "ой", "уа", "уай", "ы", "ыа", "ч", "ч",
];

fn get_cyrillic_index(key: &str) -> Option<u8> {
    hashify::tiny_map! {
        key.as_bytes(),
        "a" => 0, "b" => 1, "c" => 2, "d" => 3, "e" => 4, "f" => 5,
        "g" => 6, "h" => 7, "i" => 8, "j" => 9, "k" => 10, "l" => 11,
        "m" => 12, "n" => 13, "o" => 14, "p" => 15, "q" => 16, "r" => 17,
        "s" => 18, "t" => 19, "u" => 20, "v" => 21, "w" => 22, "x" => 23,
        "y" => 24, "z" => 25,  "qu" => 26,

        "ae" => 27, "aeo" => 28, "ai" => 29, "ao" => 30,
        "ia" => 31, "iao" => 32, "iu" => 33,
        "oe" => 34, "oei" => 35, "oi" => 36,
        "ua" => 37, "uai" => 38, "ue" => 39, "uea" => 40,

        "ch" => 41, "çh" => 42,
    }
}

pub fn get_cyrillic(key: &str) -> Option<&'static str> {
    let index = get_cyrillic_index(key)?;
    CYRILLIC_LETTERS.get(index as usize).copied()
}
