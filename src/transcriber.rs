use enum_dispatch::enum_dispatch;
use strum_macros::EnumIter;
use unicode_segmentation::UnicodeSegmentation;

use crate::casing;
use crate::langs::{burmese, thai, vietnamese};

// Enum language
#[derive(EnumIter)]
#[enum_dispatch(TranscriberTrait)]
pub enum TranscriberEnum {
    Thai(thai::Transcriber),
    Burmese(burmese::Transcriber),
    Vietnamese(vietnamese::Transcriber),
}

// Rules definition
#[derive(Copy, Clone)]
pub struct SearchRules {
    pub max_key_len: usize,
    pub prefix_start: bool,
    pub postfix_end: bool,
}

// Generic language transcriber engine
#[enum_dispatch]
pub trait TranscriberTrait {
    fn get_lang_name(&self) -> &'static str;

    fn get_search_rules(&self) -> SearchRules;

    fn get_transcription(&self, key: &str) -> Option<&'static str>;

    #[cfg(test)]
    fn get_mappings(&self) -> Vec<crate::tests::Mapping>;

    // -- DEFAULT IMPLEMENTATION --

    fn transcribe_word_segment(
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
                unreachable!();
            };

            // Check formatted key
            if let Some(transcription) = self.get_transcription(&formatted_key)
            {
                return Some(transcription);
            }
        }

        // Check original key (maybe as fallback)
        self.get_transcription(key)
    }

    fn transcribe_word(&self, input: &str) -> String {
        let mut current = input;
        let mut output = String::with_capacity(input.len() * 2);

        let SearchRules {
            max_key_len,      // Value defining the max grapheme slice length
            mut prefix_start, // Rule applied only once (at the start)
            postfix_end,      // Rule applied with extra condition (at the end)
        } = self.get_search_rules();

        while !current.is_empty() {
            let mut matched = false;

            // Collect next `max_key_len` graphemes
            let graphemes: Vec<(usize, &str)> =
                current.grapheme_indices(true).take(max_key_len).collect();

            // Iterate over graphemes backwards
            for i in (0..graphemes.len()).rev() {
                // Slice from current to farther grapheme end (longest matching)
                let (grapheme_start, grapheme) = graphemes[i];
                let grapheme_end = grapheme_start + grapheme.len();
                let segment = &current[..grapheme_end];

                // Process the segment if transcription found
                if let Some(transcription) = self.transcribe_word_segment(
                    segment,
                    prefix_start,
                    // Optimization: Conditional postfix pattern formatting only at the end
                    postfix_end && grapheme_end == current.len(),
                ) {
                    // Record && Advance && Flag
                    output.push_str(transcription);
                    current = &current[grapheme_end..];
                    matched = true;
                    // Optimization: After matching on start, no more prefix pattern formatting
                    prefix_start = false;
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

// Compile time search rules derivation
pub const fn derive_search_rules(
    rules: &'static [(&str, &str)],
) -> SearchRules {
    let mut max_key_len = 0;
    let mut prefix_start = false;
    let mut postfix_end = false;

    let mut i = 0;
    while i < rules.len() {
        let key = rules[i].0;
        let mut len = key.len();

        if key.as_bytes()[0] == b'|' {
            prefix_start = true;
            len -= 1;
        }
        if len > 0 && key.as_bytes()[key.len() - 1] == b'|' {
            postfix_end = true;
            len -= 1;
        }

        if len > max_key_len {
            max_key_len = len;
        }
        i += 1;
    }

    SearchRules {
        max_key_len,
        prefix_start,
        postfix_end,
    }
}

#[macro_export]
macro_rules! create_transcriber {
    (
        // Language name
        lang_name: $lang_name:literal,
        // Language rules
        lang_rules: { $($key:literal => $value:literal),* $(,)? },
        // Mappings (from -> to)
        mappings: { $($mapping_body:tt)* }
    ) => {
        // Create character mappings from rule tokens
        const LANG_RULES: &'static [(&'static str, &'static str)] = &[
            $( ($key, $value) ),*
        ];

        // Define transcriber struct
        #[derive(Default)]
        pub struct Transcriber;

        // Implement transcriber trait
        impl crate::transcriber::TranscriberTrait for Transcriber {
            fn get_lang_name(&self) -> &'static str {
                $lang_name
            }

            fn get_search_rules(&self) -> crate::transcriber::SearchRules {
                // Derive search rules at COMPILE TIME
                const SEARCH_RULES: crate::transcriber::SearchRules =
                    crate::transcriber::derive_search_rules(&LANG_RULES);
                SEARCH_RULES
            }

            #[cfg(test)]
            fn get_mappings(&self) -> Vec<crate::tests::Mapping> {
                $($mapping_body)*
            }

            fn get_transcription(&self, key: &str) -> Option<&'static str> {
                hashify::tiny_map! {
                    key.as_bytes(),
                    $($key => $value),*
                }
            }
        }
    };
}
