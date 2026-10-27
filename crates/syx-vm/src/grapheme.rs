//! Phase 84: Swift-aligned grapheme cluster string indexing.

use unicode_segmentation::UnicodeSegmentation;

pub fn grapheme_count(s: &str) -> usize {
    s.graphemes(true).count()
}

pub fn grapheme_at(s: &str, index: usize) -> String {
    s.graphemes(true).nth(index).unwrap_or("").to_string()
}

/// Grapheme cluster at `index`, or `(index, count)` when out of bounds (incl. negative).
pub fn try_grapheme_at(s: &str, index: i32) -> Result<String, (i32, usize)> {
    let count = grapheme_count(s);
    if index < 0 || (index as usize) >= count {
        return Err((index, count));
    }
    Ok(grapheme_at(s, index as usize))
}

pub fn grapheme_substring(s: &str, start: usize, end: usize) -> String {
    s.graphemes(true)
        .skip(start)
        .take(end.saturating_sub(start))
        .collect()
}

pub fn graphemes_vec(s: &str) -> Vec<String> {
    s.graphemes(true).map(str::to_string).collect()
}
