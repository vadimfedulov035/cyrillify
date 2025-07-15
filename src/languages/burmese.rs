pub const MAX_KEY_LEN: usize = 5;

static CYRILLIC_LETTERS: &'static [&'static str] = &[
    "а", "б", "с", "д", "э", "ф", "г", "х", "и", "з", "к", "л", "м", "н", "о", "п", "кв", "ь", "т",
    "т", "у", "в", "в", "кс", "ь", "з", "кв", // Silent "h"
    "ба", "бва", "да", "га", "за", // SH sound
    "ша", "ша", "ша", "ша", "швэй", // CH sound
    "ч", "ч", "чх", "чх", // Inversed
    "сх", "кх", "пх", "тх", // Addons
    "йва", "ньу", "йва", "пьу", // Irregular
    "джа", "тра", "ньа", "мьва", // Finals
    "э", "о", "оу", "э", "ау", "ай", "ин", "аун", "айн", "и", "и(н)", "а", "эй", "оу", "ан", "эйн",
    "оун", "а", "эй", "оу", "ан", "эйн", "оун",
];

fn get_cyrillic_index(key: &str) -> Option<u8> {
    hashify::tiny_map! {
        key.as_bytes(),
        // LATIN LETTERS + "qu" DIGRAPH
        "a" => 0, "b" => 1, "c" => 2, "d" => 3, "e" => 4,
        "f" => 5, "g" => 6, "h" => 7, "i" => 8, "j" => 9,
        "k" => 10, "l" => 11, "m" => 12, "n" => 13, "o" => 14,
        "p" => 15, "q" => 16, "r" => 17, "s" => 18, "t" => 19,
        "u" => 20, "v" => 21, "w" => 22, "x" => 23, "y" => 24,
        "z" => 25, "qu" => 26,

        // Silent "h"
        "bha" => 27, "bhwa" => 28, "dha" => 29, "gha" => 30, "jha" => 31,

        // SH sound
        "hra" => 32, "hya" => 33, "hlya" => 34, "hsya" => 35, "hrwa" => 36,

        // CH sound
        "kr" => 37, "ky" => 38, "hkr" => 39, "hky" => 40,

        // Inversed
        "hc" => 41, "hk" => 42, "hp" => 43, "ht" => 44,

        // Addons
        "ywa" => 45, "nywa" => 46,
        "rwa" => 47, "prwa" => 48,

        // Irregular
        "gya" => 49, "tra" => 50, "ngra" => 51, "mrwa" => 52,

        // Finals
        "ai" => 53, "au" => 54, "ui" => 55,
        "ak" => 56, "auk" => 57, "uik" => 58,
        "ang" => 59, "aung" => 60, "uing" => 61,
        "ac" => 62, "any" => 63,
        "at" => 64, "it" => 65, "ut" => 66,
        "an" => 67, "in" => 68, "un" => 69,
        "ap" => 70, "ip" => 71, "up" => 72,
        "am" => 73, "im" => 74, "um" => 75
    }
}

pub fn get_cyrillic(key: &str) -> Option<&'static str> {
    let index = get_cyrillic_index(key)?;
    CYRILLIC_LETTERS.get(index as usize).copied()
}
