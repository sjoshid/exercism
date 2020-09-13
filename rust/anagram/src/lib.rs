use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result = HashSet::with_capacity(possible_anagrams.len());
    let chars_in_word = char_count(word);
    for w in possible_anagrams {
        let ana = *w;
        let chars_in_ana = char_count(ana);

        // if they are equal, we move on
        let mut found_anagram = true;
        if *ana.to_lowercase() != *word.to_lowercase() && chars_in_ana.len() == chars_in_word.len()
        {
            for entry in &chars_in_word {
                let word_count = *entry.1;
                let word_char = *entry.0;

                if !(chars_in_ana.contains_key(&word_char)
                    && *chars_in_ana.get(&word_char).unwrap() == word_count)
                {
                    found_anagram = false;
                    break;
                }
            }
            if found_anagram {
                result.insert(ana);
            }
        }
    }
    result
}

pub fn char_count(s: &str) -> HashMap<char, u32> {
    let mut counts = HashMap::new();

    for w in s.to_lowercase().chars() {
        *counts.entry(w.to_owned()).or_insert(0u32) += 1u32;
    }

    counts
}
