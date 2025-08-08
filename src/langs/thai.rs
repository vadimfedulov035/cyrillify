#[cfg(test)]
use crate::tests::Mapping;

crate::create_transcriber!(
    lang_name: "Тайский",
    lang_rules: {
        // LATIN LETTERS
        "A" => "А", "B" => "Б", "C" => "К", "D" => "Д",
        "E" => "Е", "F" => "Ф", "G" => "Г", "H" => "Х",
        "I" => "И", "J" => "Ч", "K" => "К", "L" => "Л",
        "M" => "М", "N" => "Н", "O" => "О", "P" => "П",
        "Q" => "КВ", "R" => "Р", "S" => "С", "T" => "Т",
        "U" => "У", "V" => "В", "W" => "В", "X" => "КС",
        "Y" => "Й", "Z" => "З",
        // A sound
        "AE" => "Э", "AEO" => "ЭУ", "AI" => "АЙ", "AO" => "АУ",
        // I sound
        "IA" => "ИА", "IAO" => "ИО", "IU" => "ИУ",
        // O sound
        "OE" => "Е", "OEI" => "ЕЙ", "OI" => "ОЙ",
        // U sound
        "UA" => "УА", "UAI" => "УАЙ", "UE" => "Ы", "UEA" => "ЫА",
        // CH sound
        "CH" => "Ч", "ÇH" => "Ч",
    },
    mappings: {
        vec![Mapping {
            from: "A",
            to: "А",
        }]
    }
);
