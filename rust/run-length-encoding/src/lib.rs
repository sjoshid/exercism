pub fn encode(source: &str) -> String {
    let source = source.to_owned() + "0";
    let mut count = 1;
    let mut result = String::new();

    for (f, s) in source.chars().zip(source.chars().skip(1)) {
        if f == s {
            count += 1;
        } else if count == 1 {
            result = format!("{}{}", result, f);
        } else {
            result = format!("{}{}{}", result, count, f);
            count = 1;
        }
    }

    result
}

pub fn decode(source: &str) -> String {
    let source = "#".to_owned() + source;

    let indices = source
        .char_indices()
        .filter_map(|(i, c)| {
            if c.is_alphabetic() || c.is_whitespace() {
                Some(i)
            } else {
                None
            }
        })
        .collect::<Vec<usize>>();

    let mut c = 0 as usize;
    let mut result = String::new();

    for i in indices {
        if let Some(s) = source.get((c + 1)..i) {
            match s.is_empty() {
                true => {
                    let char_with_one_count = source.get(i..(i + 1)).unwrap();
                    result += char_with_one_count;
                }
                false => {
                    if let Ok(v) = s.parse::<usize>() {
                        let char_to_repeat = source.get(i..(i + 1)).unwrap();
                        result += char_to_repeat.repeat(v).as_str();
                    }
                }
            }
            c = i;
        }
    }
    result
}
