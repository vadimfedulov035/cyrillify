use unicode_segmentation::UnicodeSegmentation;

/// Transliterate separate word to cyrill alphabet, fall back to rewriting it
pub fn cirillify_word<F>(input: &str, max_len: usize, get_cyrill: F) -> String
where
    F: Fn(&str) -> Option<&'static str>,
{
    // Pre-allocate string with a reasonable guess to avoid re-allocations
    let mut output = String::with_capacity(input.len() * 2);
    let mut current = input;

    while !current.is_empty() {
        let mut matched = false;

        // Collect next `max_len` graphemes
        let graphemes: Vec<(usize, &str)> = current.grapheme_indices(true).take(max_len).collect();

        // Check from longest to shortest match
        for i in (0..graphemes.len()).rev() {
            // Slice till the i-th grapheme end
            let (grapheme_start, grapheme) = graphemes[i];
            let grapheme_end = grapheme_start + grapheme.len();
            let slice = &current[..grapheme_end];

            // On match, cyrillify and advance
            if let Some(cyrill) = get_cyrill(slice) {
                output.push_str(cyrill);
                current = &current[grapheme_end..];
                matched = true;
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

/// Helper function to capitalize the first grapheme
fn capitalize_first_grapheme(s: &str) -> String {
    let mut graphemes = s.graphemes(true);
    if let Some(first) = graphemes.next() {
        first.to_uppercase() + graphemes.as_str()
    } else {
        String::new()
    }
}

/// Transliterate separate segment to cyrill alphabet
fn cyrillify_segment<F>(segment: &str, max_len: usize, get_cyrill: &F, final_result: &mut String)
where
    F: Fn(&str) -> Option<&'static str>,
{
    if segment.is_empty() {
        return;
    }

    if segment.chars().next().unwrap().is_alphabetic() {
        // --- Handle WORD ---

        // Note if word is capitalized
        let was_capitalized = segment.chars().next().unwrap().is_uppercase();

        // Tranliterate lowercased word
        let lowercase_word = segment.to_lowercase();
        let cyrill_word = cirillify_word(&lowercase_word, max_len, get_cyrill);

        // Capitalize the result if the original was capitalized.
        if was_capitalized {
            final_result.push_str(&capitalize_first_grapheme(&cyrill_word));
        } else {
            final_result.push_str(&cyrill_word);
        }
    } else {
        // --- Handle SEPARATOR (e.g., " ", "-") ---
        final_result.push_str(segment);
    }
}

/// Transliterate an input string to to cyrill alphabet by splitting it into words and separators
pub fn cyrillify<F>(input: &str, max_len: usize, get_cyrill: F) -> String
where
    F: Fn(&str) -> Option<&'static str>,
{
    let mut final_result = String::with_capacity(input.len() * 2);
    let mut last_boundary = 0;

    // Determine starting character type of grapheme
    let mut last_was_alphabetic = input
        .graphemes(true)
        .next()
        .map_or(false, |g| g.chars().next().unwrap().is_alphabetic());

    // Iterate over graphemes
    for (i, grapheme) in input.grapheme_indices(true) {
        // Check for change of character type
        let current_is_alphabetic = grapheme.chars().next().unwrap().is_alphabetic();
        if current_is_alphabetic != last_was_alphabetic {
            // Process the segment on change
            let segment = &input[last_boundary..i];
            cyrillify_segment(segment, max_len, &get_cyrill, &mut final_result);

            // Update state for the next segment
            last_boundary = i;
            last_was_alphabetic = current_is_alphabetic;
        }
    }

    // After the loop, process the final remaining segment of the string.
    cyrillify_segment(
        &input[last_boundary..],
        max_len,
        &get_cyrill,
        &mut final_result,
    );

    final_result
}
