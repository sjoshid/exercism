use std::ops::{Add, Sub};

pub struct PascalsTriangle {
    number_of_rows: u32
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle {
            number_of_rows: row_count
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut result = Vec::<Vec<u32>>::new();

        for i in 0..self.number_of_rows {
            let mut row = vec![0; i.add(1) as usize];
            row[0] = 1;
            *row.last_mut().unwrap() = 1;

            if row.len() > 2 {
                let previous_row: &Vec<u32> = result[i.sub(1) as usize].as_ref();
                let row_len = row.len();

                for (i, e) in row.iter_mut().enumerate() {
                    if i > 0 && i < row_len.sub(1) {
                        *e = previous_row[i.sub(1)] + previous_row[i];
                    }
                }
            }

            result.push(row);
        }

        result
    }
}
