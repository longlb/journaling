use chrono::prelude::*;

// Wrapper for the Date struct.
pub struct SuperDate {
    date: Date<Local>,
}

impl SuperDate {
    // Today's date.
    pub fn today() -> Self {
        Self {
            date: Local::today(),
        }
    }

    // Today's date + the days given.
    pub fn days(days: i64) -> Self {
        Self {
            date: Local::today() + chrono::Duration::days(days),
        }
    }

    // Date processed from a string.
    pub fn process(input: &str) -> Self {
        let date: Vec<u32> = input
            .split('-')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        Self {
            date: Local.ymd(date[0] as i32, date[1], date[2]),
        }
    }

    // The date's year as a string.
    pub fn year(&self) -> String {
        self.date.year().to_string()
    }

    // The date's month as a string, converted from a number.
    pub fn month(&self) -> String {
        match self.date.month() {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => "not a month",
        }
        .to_string()
    }

    // The date's day as a string.
    pub fn day(&self) -> String {
        self.date.day().to_string()
    }
}
