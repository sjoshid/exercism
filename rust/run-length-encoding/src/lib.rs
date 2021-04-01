pub fn encode(source: &str) -> String {
    if source.is_empty() {
        String::from("")
    } else {
        let mut source = source.to_string();
        source.push('0');

        let mut cc = String::from(source.get(0..1).unwrap());
        let mut count = 1;
        let mut result = String::from("");

        for e in source.chars().skip(1) {
            let c = e.to_string();

            if c == cc {
                count += 1;
            } else if count == 1 {
                result += format!("{}", cc).as_str();
                count = 1;
                cc = c;
            } else {
                result += format!("{}{}", count, cc).as_str();
                count = 1;
                cc = c;
            }
        }

        result
    }
}

pub fn decode(source: &str) -> String {
    unimplemented!("Return the run-length decoding of {}.", source);
}
