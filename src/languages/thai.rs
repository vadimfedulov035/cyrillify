pub const MAX_KEY_LEN: usize = 3;

static CYRILLS: &'static [&'static str] = &[
    "а", "б", "к", "д", "е", "ф", "г", "х", "и", "к", "ч", "л", "м", "н", "о", "п", "кв", "р", "с",
    "т", "у", "в", "в", "кс", "й", "з", "э", "эу", "ай", "ау", "иа", "ио", "иу", "е", "ей", "ой",
    "уа", "уай", "ы", "ыа", "ч", "ч",
];

fn get_cyrill_index(key: &str) -> Option<u8> {
    hashify::tiny_map! {
        key.as_bytes(),
        "a" => 0, "b" => 1, "c" => 2, "d" => 3, "e" => 4, "f" => 5,
        "g" => 6, "h" => 7, "i" => 8, "j" => 9, "k" => 10, "l" => 11,
        "m" => 12, "n" => 13, "o" => 14, "p" => 15, "q" => 16, "r" => 17,
        "s" => 18, "t" => 19, "u" => 20, "v" => 21, "w" => 22, "x" => 23,
        "y" => 24, "z" => 25,

        "ae" => 26, "aeo" => 27, "ai" => 28, "ao" => 29,
        "ia" => 30, "iao" => 31, "iu" => 32,
        "oe" => 33, "oei" => 34, "oi" => 35,
        "ua" => 37, "uai" => 38, "ue" => 39, "uea" => 40,

        "ch" => 40, "çh" => 41,
    }
}

pub fn get_cyrill(key: &str) -> Option<&'static str> {
    let index = get_cyrill_index(key)?;
    CYRILLS.get(index as usize).copied()
}
