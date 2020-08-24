use chrono::prelude::*;
use regex::Regex;

pub struct EntryDay {
    date: Date<Local>,
}

impl EntryDay {
    pub fn new() -> Self {
        Self {
            date: Local::today(),
        }
    }

    pub fn input(&mut self, input: &str) {
        let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
        self.date = match input {
            _ if re.is_match(input) => EntryDay::processed_date(input),
            "yesterday" => self.date - chrono::Duration::days(1),
            "tomorrow" => self.date + chrono::Duration::days(1),
            "random" => self.date,
            _ => self.date,
        }
    }

    fn processed_date(input: &str) -> Date<Local> {
        let date: Vec<u32> = input
            .split('-')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        Local.ymd(date[0] as i32, date[1], date[2])
    }

    pub fn year(&self) -> String {
        self.date.year().to_string()
    }

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
            _ => "Not a month",
        }
        .to_string()
    }

    pub fn day(&self) -> String {
        self.date.day().to_string()
    }
}
