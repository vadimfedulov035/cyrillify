use crate::transcriber::{TranscriberEnum, TranscriberTrait};
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct Mapping<'a> {
    pub from: &'a str,
    pub to: &'a str,
}

#[test]
fn test_langs() {
    for transcriber in TranscriberEnum::iter() {
        test_lang(transcriber);
    }
}

fn test_lang<T: TranscriberTrait>(transcriber: T) {
    let mappings = transcriber.get_mappings();
    for mapping in mappings {
        let result = transcriber.transcribe(mapping.from);
        assert_eq!(
            result,
            mapping.to,
            // This provides a much more detailed error message upon failure
            "'{}': '{}' -> '{}', wrong transcription '{:?}'",
            transcriber.get_lang_name(),
            mapping.from,
            mapping.to,
            result,
        );
    }
}
