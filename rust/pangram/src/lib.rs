use std::collections::{HashSet, BTreeSet};

struct AsciiChars(String);

//String::from("abcdefghijklmnopqrstuvwxyz");

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let letters: HashSet<_> = sentence
        .chars()
        .filter(char::is_ascii_alphabetic)
        .map(|c| c.to_ascii_lowercase())
        .collect();

    letters.len() == 26
}
