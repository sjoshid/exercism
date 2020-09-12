pub fn brackets_are_balanced(string: &str) -> bool {
    let mut char_vec = Vec::new();

    for c in string.chars() {
        match c {
            '[' => char_vec.push('['),
            ']' => {
                let last = char_vec.last();
                if last.is_none() || *last.unwrap() != '[' {
                    return false;
                } else {
                    char_vec.pop();
                }
            },
            '{' => char_vec.push('{'),
            '}' => {
                let last = char_vec.last();
                if last.is_none() || *last.unwrap() != '{' {
                    return false;
                } else {
                    char_vec.pop();
                }
            },
            '(' => char_vec.push('('),
            ')' => {
                let last = char_vec.last();
                if last.is_none() || *last.unwrap() != '(' {
                    return false;
                } else {
                    char_vec.pop();
                }
            },
            _ => {}
        }
    }

    char_vec.is_empty()
}

fn is_valid(chech_char: &char, last_char: Option<&char>) -> bool {
    if last_char.is_none() || *last_char.unwrap() != *chech_char {
        false
    } else {
        true
    }
}
