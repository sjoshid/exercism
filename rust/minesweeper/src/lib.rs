pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result = Vec::new();
    for (i, e) in minefield.iter().enumerate() {
        let minefield_str = *e;
        let mut result_string = String::from("");
        for (j, c) in minefield_str.chars().enumerate() {
            if c != '*' {
                let mines_around_me = get_mines_count(i, j, minefield);
                if mines_around_me > 0 {
                    result_string = format!("{}{}", result_string, mines_around_me);
                } else {
                    result_string = format!("{} ", result_string);
                }
            } else {
                result_string = format!("{}*", result_string);
            }
        }
        result.push(result_string);
    }
    result
}

fn get_mines_count(r: usize, c: usize, minefield: &[&str]) -> u8 {
    let mut mine_count = 0;
    // edges
    mine_count += is_mine(Some(r), c.checked_add(1), minefield);
    mine_count += is_mine(Some(r), c.checked_sub(1), minefield);
    mine_count += is_mine(r.checked_sub(1), Some(c), minefield);
    mine_count += is_mine(r.checked_add(1), Some(c), minefield);

    // corners
    mine_count += is_mine(r.checked_sub(1), c.checked_sub(1), minefield);
    mine_count += is_mine(r.checked_sub(1), c.checked_add(1), minefield);
    mine_count += is_mine(r.checked_add(1), c.checked_add(1), minefield);
    mine_count += is_mine(r.checked_add(1), c.checked_sub(1), minefield);
    mine_count
}

fn is_mine(r: Option<usize>, c: Option<usize>, minefield: &[&str]) -> u8 {
    match (r, c) {
        (Some(r), Some(c)) if r < minefield.len() && c < minefield[0].len() => {
            let row = minefield[r];
            let value = row.get(c..(c + 1)).unwrap();

            if value == "*" {
                1
            } else {
                0
            }
        }
        (_, _) => 0, //at least one invalid
    }
}
