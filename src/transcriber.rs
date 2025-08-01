use enum_dispatch::enum_dispatch;
use strum_macros::EnumIter;
use unicode_segmentation::UnicodeSegmentation;

use crate::casing;
use crate::langs::{burmese, thai, vietnamese};

// Enum language
#[derive(EnumIter)]
#[enum_dispatch(TranscriberTrait)]
pub enum TranscriberEnum {
    Burmese(burmese::Transcriber),
    Thai(thai::Transcriber),
    Vietnamese(vietnamese::Transcriber),
}

// Rules definition
#[derive(Copy, Clone)]
pub struct FormatRules {
    pub prefix_start: bool,
    pub postfix_end: bool,
}

// Generic language transcriber
#[enum_dispatch]
pub trait TranscriberTrait {
    // -- METHODS TO IMPLEMENT --
    /// Returns language name
    fn get_lang_name(&self) -> &'static str;

    /// Returns the maximum key length to handle
    fn get_max_key_len(&self) -> usize;

    /// Returns format rules applied to key
    fn get_format_rules(&self) -> FormatRules;

    /// Returns transcription for given uppercase key
    fn get_transcription_str(&self, key: &str) -> Option<&'static str>;

    // -- DEFAULT IMPLEMENTATION --

    /// Transcribes key using its config
    fn get_transcription(
        &self,
        key: &str,
        prefix_start: bool,
        postfix_end: bool,
    ) -> Option<&'static str> {
        // Format key if needed
        if prefix_start || postfix_end {
            let formatted_key = if prefix_start && postfix_end {
                format!("|{}|", key)
            } else if prefix_start {
                format!("|{}", key)
            } else if postfix_end {
                format!("{}|", key)
            } else {
                panic!();
            };

            // Check formatted key
            if let Some(transcription) =
                self.get_transcription_str(&formatted_key)
            {
                return Some(transcription);
            }
        }

        // Check original key (right away or as a fallback)
        self.get_transcription_str(key)
    }

    /// Transcribes word to cyrillic alphabet, falls back to rewriting it
    fn transcribe_word(&self, input: &str) -> String {
        let mut current = input;
        let mut output = String::with_capacity(input.len() * 2);

        let max_key_len = self.get_max_key_len();
        let FormatRules {
            mut prefix_start,
            mut postfix_end,
        } = self.get_format_rules();

        while !current.is_empty() {
            let mut matched = false;

            // Collect next `max_key_len` graphemes
            let graphemes: Vec<(usize, &str)> =
                current.grapheme_indices(true).take(max_key_len).collect();

            // Iterate over graphemes backwards
            for i in (0..graphemes.len()).rev() {
                // Slice from current position to i-th grapheme end
                let (grapheme_start, grapheme) = graphemes[i];
                let grapheme_end = grapheme_start + grapheme.len();
                let segment = &current[..grapheme_end];

                // On match, transcribe and advance
                if let Some(transcription) =
                    self.get_transcription(segment, prefix_start, postfix_end)
                {
                    output.push_str(transcription);
                    current = &current[grapheme_end..];
                    matched = true;
                    prefix_start = false;
                    postfix_end = false;
                    break;
                }
            }

            // On no match, copy as-is
            if !matched {
                if let Some(first_grapheme) = current.graphemes(true).next() {
                    output.push_str(first_grapheme);
                    current = &current[first_grapheme.len()..];
                }
            }
        }

        output
    }

    /// Transcribes segment case sensitive
    fn transcribe_segment(&self, segment: &str, result: &mut String) {
        // -- SEGMENT HANDLING --
        // Discard empty segment
        if segment.is_empty() {
            return;
        }

        // Transcribe non-alphabetic segment
        if !segment.chars().next().unwrap().is_alphabetic() {
            result.push_str(segment);
            return;
        }

        // -- WORD HANDLING --
        let word = segment;

        // 1. Record original case
        let case = casing::record_case(&word);

        // 2. Transcribe uppercased segment
        let word_upper = word.to_uppercase();
        let word_transcribed = self.transcribe_word(&word_upper);

        // 3. Reapply original case
        let word_cased = casing::reapply_case(&word_transcribed, case);
        result.push_str(&word_cased);
    }

    /// Transcribe input: transcribe_segment -> transcribe_word
    fn transcribe(&self, input: &str) -> String {
        let mut result = String::with_capacity(input.len() * 2);
        let mut last_boundary = 0;

        // Define if first segment alphabetic
        let mut last_was_alphabetic = input
            .graphemes(true)
            .next()
            .map_or(false, |g| g.chars().next().unwrap().is_alphabetic());

        // Transcribe segments based on boundaries
        for (i, grapheme) in input.grapheme_indices(true) {
            let current_is_alphabetic =
                grapheme.chars().next().unwrap().is_alphabetic();

            if current_is_alphabetic != last_was_alphabetic {
                let segment = &input[last_boundary..i];
                self.transcribe_segment(segment, &mut result);
                last_boundary = i;
                last_was_alphabetic = current_is_alphabetic;
            }
        }

        // Transcribe segment after the boundary
        self.transcribe_segment(&input[last_boundary..], &mut result);

        result
    }
}
