use unicode_segmentation::UnicodeSegmentation;

#[derive(PartialEq)]
pub enum Case {
    Lower,
    Title,
    Upper,
    Mixed,
}

/// Converts string to Title
fn titleize(s: &str) -> String {
    let mut graphemes = s.graphemes(true);
    let first = graphemes.next().unwrap();
    first.to_uppercase() + graphemes.as_str()
}

/// Reapplies original word case
pub fn reapply_case(word: &str, case: Case) -> String {
    match case {
        Case::Lower => word.to_lowercase(),
        Case::Upper => word.to_uppercase(),
        Case::Title | Case::Mixed => titleize(word),
    }
}

/// Records original word case
pub fn record_case(input: &str) -> Case {
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
