pub fn brackets_are_balanced(string: &str) -> bool {
    let mut char_vec = Vec::new();

    for c in string.chars() {
        match c {
            '[' => char_vec.push('['),
            ']' => {
                let last = char_vec.pop();
                if last.is_none() || last.unwrap() != '[' {
                    return false;
                }
            },
            '{' => char_vec.push('{'),
            '}' => {
                let last = char_vec.pop();
                if last.is_none() || last.unwrap() != '{' {
                    return false;
                }
            },
            '(' => char_vec.push('('),
            ')' => {
                let last = char_vec.pop();
                if last.is_none() || last.unwrap() != '(' {
                    return false;
                }
            },
            _ => {}
        }
    }

    char_vec.is_empty()
}
