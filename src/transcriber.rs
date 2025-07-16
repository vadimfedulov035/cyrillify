use unicode_segmentation::UnicodeSegmentation;

/// Transcribes word to cyrillic alphabet, falls back to rewriting it
pub fn transcribe_word<F>(
    input: &str,
    max_len: usize,
    get_transcription: F,
    mark_word_start: bool,
) -> String
where
    F: Fn(&str, bool) -> Option<&'static str>,
{
    let mut current = input;
    let mut output = String::with_capacity(input.len() * 2);

    let mut mark_word_start_this = mark_word_start;

    while !current.is_empty() {
        let mut matched = false;

        // Collect next `max_len` graphemes
        let graphemes: Vec<(usize, &str)> =
            current.grapheme_indices(true).take(max_len).collect();

        // Check from longest to shortest match
        for i in (0..graphemes.len()).rev() {
            // Slice from current to i-th grapheme end
            let (grapheme_start, grapheme) = graphemes[i];
            let grapheme_end = grapheme_start + grapheme.len();
            let segment = &current[..grapheme_end];

            // On match, transcribe and advance
            if let Some(transcription) =
                get_transcription(segment, mark_word_start_this)
            {
                output.push_str(transcription);
                current = &current[grapheme_end..];
                matched = true;
                mark_word_start_this = false;
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

#[derive(PartialEq)]
enum Case {
    Lower,
    Title,
    Upper,
    Mixed,
}

/// Determines word case
fn get_case(input: &str) -> Case {
    let mut chars = input.chars();

    // Get first character for sure
    let first = chars.next().unwrap();

    // Make first assumption for case
    if first.is_lowercase() {
        // Check for lowercase consistency
        if chars.any(|c| c.is_uppercase()) {
            Case::Mixed
        } else {
            Case::Lower
        }
    } else {
        // Check for uppercase consistency
        let mut has_lowercase = false;
        let mut has_uppercase = false;

        // Identify internal cases
        for c in chars {
            if c.is_uppercase() {
                has_uppercase = true;
            } else if c.is_lowercase() {
                has_lowercase = true
            }

            if has_uppercase && has_lowercase {
                break;
            }
        }

        // Identify general case based on the internal ones
        if has_lowercase && has_uppercase {
            Case::Mixed
        } else if has_uppercase {
            Case::Upper
        } else if has_lowercase {
            Case::Title
        } else {
            Case::Title
        }
    }
}

/// Transcribes segment case sensitive
fn transcribe_segment(
    segment: &str,
    max_len: usize,
    get_transcription: &dyn Fn(&str, bool) -> Option<&'static str>,
    mark_word_start: bool,
    result: &mut String,
) {
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

    // 0. Define Titleize as default style applied
    let titleize = |s: &str| -> String {
        let mut graphemes = s.graphemes(true);
        if let Some(first) = graphemes.next() {
            first.to_uppercase() + graphemes.as_str()
        } else {
            String::new()
        }
    };

    // 1. Record original case
    let case = get_case(word);

    // 2. Transcribe lowercased segment
    let lowercase_word = word.to_lowercase();
    let transcribed_word = transcribe_word(
        &lowercase_word,
        max_len,
        get_transcription,
        mark_word_start,
    );

    // 3. Apply original case
    match case {
        Case::Lower => {
            result.push_str(&transcribed_word);
        }
        Case::Title => {
            result.push_str(&titleize(&transcribed_word));
        }
        Case::Upper => {
            result.push_str(&transcribed_word.to_uppercase());
        }
        // Fall back to TitleCase
        Case::Mixed => {
            result.push_str(&titleize(&transcribed_word));
        }
    }
}

/// Transcribe input: transcribe_segment -> transcribe_word
pub fn transcribe(
    input: &str,
    max_len: usize,
    get_transcription: impl Fn(&str, bool) -> Option<&'static str>,
    mark_word_start: bool,
) -> String {
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
            transcribe_segment(
                segment,
                max_len,
                &get_transcription,
                mark_word_start,
                &mut result,
            );
            last_boundary = i;
            last_was_alphabetic = current_is_alphabetic;
        }
    }

    // Transcribe segment after the boundary
    transcribe_segment(
        &input[last_boundary..],
        max_len,
        &get_transcription,
        mark_word_start,
        &mut result,
    );

    result
}
