pub const MAX_KEY_LEN: usize = 4;

fn get_transcription_str(key: &str) -> Option<&'static str> {
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

        // Inversed
        "hc" => "сх",
        "hk" => "кх",
        "hp" => "пх",
        "ht" => "тх",

        // Y combinations
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

pub fn get_transcription(
    key: &str,
    mark_word_start: bool,
) -> Option<&'static str> {
    // Try to check prefixed key first
    if mark_word_start {
        let prefixed_key = format!("|{}", key);
        if let Some(transcription) = get_transcription_str(&prefixed_key) {
            return Some(transcription);
        }
    }

    // Only then fall back to non prefixed key
    let Some(transcription) = get_transcription_str(key) else {
        return None;
    };
    Some(transcription)
}
