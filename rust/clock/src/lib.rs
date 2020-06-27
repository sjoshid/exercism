use std::fmt::{Display, Formatter, Result};

#[derive(PartialEq, Debug)]
pub struct Clock {
    minutes: i32,
    hours: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = (hours * 60) + minutes;
        let minutes_in_a_day = 24 * 60;
        let mut remaining_minutes = total_minutes % minutes_in_a_day;
        if remaining_minutes < 0 {
            remaining_minutes += minutes_in_a_day;
        }
        let hours = remaining_minutes / 60;
        let minutes = remaining_minutes % 60;

        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{:0>2}:{:0>2}",
            self.hours.to_string(),
            self.minutes.to_string()
        )
    }
}
