use chrono::prelude::*;
use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    // Get today's date.
    let today = Local::today();
    let path = Path::new("/home/longlb/Documents/journal");

    // Taking the argument following the journal command.
    let args: Vec<String> = env::args().collect();
    let second = match args.len() {
        1 => "today",
        _ => &args[1],
    };
    let attachment = match second {
        "today" => today.format("%Y-%m-%d"),
        "tomorrow" => (today + chrono::Duration::days(1)).format("%Y-%m-%d"),
        "yesterday" => (today - chrono::Duration::days(1)).format("%Y-%m-%d"),
        _ => today.format("%Y-%m-%d"),
    };

    let entry = format!("{}/{}", path.to_str().unwrap(), attachment);
    println!("{}", entry);

    // Execute combined command
    Command::new("mousepad")
        .arg(entry)
        .spawn()
        .expect("Cannot open journal");
}
