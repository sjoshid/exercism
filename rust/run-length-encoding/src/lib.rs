pub fn encode(source: &str) -> String {
    let source = source.to_owned() + "0";
    let mut count = 1;
    let mut result = String::from("");

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
    unimplemented!("Return the run-length decoding of {}.", source);
}
