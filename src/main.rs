use chrono::prelude::*;
use std::path::PathBuf;
use std::process::Command;
use std::{env, io};

// TODO
// Opening an entry by specific date
// Opening a random journal entry
// Optimization
// Uses system text editor rather than mousepad?
// Functionality for fully usable on any system? (Directories and such)

struct Temp {
    base_path: PathBuf,
    day: Date<Local>,
}

impl Temp {
    fn new() -> Self {
        Self {
            base_path: PathBuf::from("/home/longlb/Documents/journal"),
            day: get_day(),
        }
    }
}

// println!(
//     "Opening journal entry for {}, {} {}...",
//     day.year(),
//     month(day.month()),
//     day.day()
// );

fn main() -> io::Result<()> {
    // Initialize the general journal path and the entry date based on user input..
    let mut path = PathBuf::from("/home/longlb/Documents/journal");
    let day = get_day();

    // Enter the day's year and month directories.
    check_add_dir(&mut path, day.year().to_string())?;
    check_add_dir(&mut path, month(day.month()))?;

    // Add the day to the path.
    path.push(day.day().to_string());

    // Execute the combined command.
    execute("mousepad", path_str(&path))?;
    Ok(())
}

// fn main() -> io::Result<()> {}

// Return the day based on the user's input.
fn get_day() -> Date<Local> {
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
    day
}

// Attach a directory to the given path and generate it if if doesn't exist.
fn check_add_dir(path: &mut PathBuf, dir: String) -> io::Result<()> {
    path.push(dir);
    if !path.is_dir() {
        execute("mkdir", path_str(path))?;
    }
    Ok(())
}

// Execute a linux command with a single argument.
fn execute(command: &str, arg: &str) -> io::Result<()> {
    Command::new(command).arg(arg).spawn()?;
    Ok(())
}

// ----- Minor Helper Functions -----
// Get the &str representation of a path.
fn path_str(path: &PathBuf) -> &str {
    path.to_str().expect("Path has invalid unicode")
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
