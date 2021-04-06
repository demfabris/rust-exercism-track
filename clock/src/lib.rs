use std::fmt;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut total = hours * 60 + minutes;

        if total < 0 {
            total = 1440 * (1 + (total / 1440).abs() as i32) + total;
        }

        Self {
            hours: ((total / 60) % 24) as i32,
            minutes: (total % 60) as i32,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut total = (self.hours * 60) + self.minutes + minutes;

        if total < 0 {
            total = 1440 * (1 + (total / 1440).abs() as i32) + total;
        }

        Self {
            hours: ((total / 60) % 24) as i32,
            minutes: (total % 60) as i32,
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

fn main() {
    println!("{:?}", Clock::new(-31, 3).to_string());
    unimplemented!();
}
