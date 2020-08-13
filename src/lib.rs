use chrono::prelude::*;
// use regex::Regex;
use std::process::Command;
use std::{env, io};

mod entrypath;
use entrypath::EntryPath;
// let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
// assert!(re.is_match("2014-01-01"));
// let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
//     if args.len() > 1 {
//         if re.is_match(&args[1]) {
//         }

pub fn run() -> io::Result<()> {
    // Initialize the general journal path and the entry date based on user input..
    let mut path = EntryPath::new();
    let mut day = Local::today();
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        day = day
            + match args[1].as_str() {
                "tomorrow" => chrono::Duration::days(1),
                "yesterday" => chrono::Duration::days(-1),
                _ => chrono::Duration::days(0),
            };
    }

    // Enter the day's year and month directories.
    path.add_dir(day.year().to_string())?;
    path.add_dir(month(day.month()))?;

    // Add the day to the path.
    path.add_file(day.day().to_string())?;

    // Execute the combined command.
    execute("mousepad", path.to_str()?)?;
    Ok(())
}

// Return the day based on the user's input.
// fn get_day() -> Date<Local> {
//     let mut day = Local::today();
//     let args: Vec<String> = env::args().collect();
//     if args.len() > 1 {
//         day = day
//             + match args[1].as_str() {
//                 "tomorrow" => chrono::Duration::days(1),
//                 "yesterday" => chrono::Duration::days(-1),
//                 _ => chrono::Duration::days(0),
//             };
//     }
//     day
// }

// ----- Helper Functions -----
// Execute a linux command with a single argument.
fn execute(command: &str, arg: &str) -> io::Result<()> {
    Command::new(command).arg(arg).spawn()?;
    Ok(())
}

// Return the month name of valid month number.
fn month(num: u32) -> String {
    match num {
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
