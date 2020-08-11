use chrono::prelude::*;
use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Initialize the general journal path and today's date.
    let mut base_path = PathBuf::from("/home/longlb/Documents/journal");
    let mut day = Local::today();

    // Adjust the day based on the extra arguments given, if there are any.
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        day = day
            + match args[1].as_str() {
                "tomorrow" => chrono::Duration::days(1),
                "yesterday" => chrono::Duration::days(-1),
                _ => chrono::Duration::days(0),
            };
    }

    // Add the year to the path.
    base_path.push(day.year().to_string());
    if !base_path.is_dir() {
        execute("mkdir", base_path.to_str().expect("Not valid"))
    }

    // Add the month to the path.
    base_path.push(month(day.month()));
    if !base_path.is_dir() {
        execute("mkdir", base_path.to_str().expect("Not valid"))
    }

    // Add the day to the path.
    base_path.push(day.day().to_string());

    // Execute the combined command
    execute("mousepad", base_path.to_str().expect("Not valid"));
    println!(
        "Opening journal entry for {}, {} {}...",
        day.year(),
        month(day.month()),
        day.day()
    );
}

fn execute(command: &str, arg: &str) {
    Command::new(command)
        .arg(&arg)
        .spawn()
        .expect(&format!("Command {} {} failed", command, arg));
}

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
