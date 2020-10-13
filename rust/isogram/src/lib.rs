use std::collections::{HashSet, HashMap};

pub fn check(candidate: &str) -> bool {
    let mut result_map: HashSet<char> = HashSet::new();

    let mut result = String::from(candidate.to_uppercase());
    result = result.replace("-","");
    result = result.replace(" ","");

    for c in result.chars() {
        if !result_map.insert(c) {
            return false;
        }
    }
    true
}
