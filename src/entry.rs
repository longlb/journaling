use super::execute;
use chrono::prelude::*;
use rand::seq::IteratorRandom;
use regex::Regex;
use std::io;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

pub struct Entry {
    path: PathBuf,
    date: Date<Local>,
}

impl Entry {
    // Generate a journal entry path.
    pub fn new() -> Self {
        Self {
            // It's possible to use dirs crate for home directory variable rather than hard code.
            // But I'll leave it like this for now, since I'm the only one using it.
            path: PathBuf::from("/home/longlb/Documents/journal"),
            // Default to the day that the command is called.
            date: Local::today(),
        }
    }

    // pub fn input(input: &str) -> Self {
    //     Self {
    //         // It's possible to use dirs crate for home directory variable rather than hard code.
    //         // But I'll leave it like this for now, since I'm the only one using it.
    //         path: PathBuf::from("/home/longlb/Documents/journal"),
    //         // Default to the day that the command is called.
    //         date: Local::today(),
    //     }
    // }

    // Adjust the journal entry date based on the given input.
    pub fn input(&mut self, input: &str) {
        let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
        self.date = match input {
            _ if re.is_match(input) => Entry::processed_date(input),
            "yesterday" => self.date - chrono::Duration::days(1),
            "tomorrow" => self.date + chrono::Duration::days(1),
            "random" => self.random_date(),
            _ => self.date,
        }
    }

    // Return a str of the full path.
    pub fn to_str(&mut self) -> io::Result<&str> {
        self.path
            .to_str()
            .ok_or(Error::new(ErrorKind::Other, "Path has invalid unicode"))
    }

    // -------------------------------------------------------

    pub fn add_dir(&mut self, dir: String) -> io::Result<()> {
        self.path.push(dir);
        if !self.path.is_dir() {
            execute("mkdir", self.to_str()?)?;
        }
        Ok(())
    }

    pub fn add_file(&mut self, file: String) -> io::Result<()> {
        self.path.push(file);
        if !self.path.is_file() {
            execute("touch", self.to_str()?)?;
        }
        Ok(())
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

    fn processed_date(input: &str) -> Date<Local> {
        let date: Vec<u32> = input
            .split('-')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        Local.ymd(date[0] as i32, date[1], date[2])
    }

    fn random_date(&mut self) -> Date<Local> {
        let mut rng = rand::thread_rng();

        let x = self
            .path
            .read_dir()
            .expect("read_dir call failed")
            .choose(&mut rng)
            .unwrap()
            .unwrap()
            .path()
            .read_dir()
            .expect("read_dir call failed")
            .choose(&mut rng)
            .unwrap()
            .unwrap()
            .path()
            .read_dir()
            .expect("read_dir call failed")
            .choose(&mut rng)
            .unwrap()
            .unwrap()
            .path();
        let mut temp = x.components().rev();

        let day = temp.next().unwrap().as_os_str().to_str().unwrap();
        let month = temp.next().unwrap().as_os_str().to_str().unwrap();
        let year = temp.next().unwrap().as_os_str().to_str().unwrap();

        println!("{:?} {:?} {:?}", year, month, day);
        Local.ymd(
            year.parse::<i32>().unwrap(),
            rev_month(month),
            day.parse::<u32>().unwrap(),
        )
    }
}

fn rev_month(input: &str) -> u32 {
    match input {
        "January" => 1,
        "February" => 2,
        "March" => 3,
        "April" => 4,
        "May" => 5,
        "June" => 6,
        "July" => 7,
        "August" => 8,
        "September" => 9,
        "October" => 10,
        "November" => 11,
        "December" => 12,
        _ => 0,
    }
}
