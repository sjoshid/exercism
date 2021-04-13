// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let mut lines = input.split("\n").collect::<Vec<&str>>();
    let mut results = Vec::new();

    for line in lines.chunks(4) {
        let mut digits_on_same_line = String::new();
        if line.len() % 4 != 0 {
            return Err(Error::InvalidRowCount(line.len()));
        } else if !(line[0].len() == line[1].len() && line[1].len() == line[2].len() && line[2].len() == line[3].len())
            || (line[0].len() % 3 != 0)
            || (line[1].len() % 3 != 0)
            || (line[2].len() % 3 != 0)
            || (line[3].len() % 3 != 0)
        {
            return Err(Error::InvalidColumnCount(line[0].len()));
        } else {
            for i in (0..line[0].len()).step_by(3) {
                let current = *line.get(0).unwrap(); // row 1
                let f = &current[i..(i+3)];
                let current = *line.get(1).unwrap(); // row 2
                let s = &current[i..(i+3)];
                let current = *line.get(2).unwrap(); // row 3
                let t = &current[i..(i+3)];
                let current = *line.get(3).unwrap(); // row 4
                let l = &current[i..(i+3)];

                let string_to_check = f.to_owned() + s + t + l;
                let parsed = check_for_digit(string_to_check.as_str());
                digits_on_same_line += parsed;
            }
            results.push(digits_on_same_line);
        }
    }
    return Ok(results.join(","));
}

fn check_for_digit(check_for_digit: &str) -> &str {
    let digit = match check_for_digit {
        " _ | ||_|   " => "0",
        "     |  |   " => "1",
        " _  _||_    " => "2",
        " _  _| _|   " => "3",
        "   |_|  |   " => "4",
        " _ |_  _|   " => "5",
        " _ |_ |_|   " => "6",
        " _   |  |   " => "7",
        " _ |_||_|   " => "8",
        " _ |_| _|   " => "9",
        _ => "?",
    };
    digit
}
