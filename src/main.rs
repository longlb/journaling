use chrono::prelude::*;
use std::env;
use std::process::Command;

fn main() {
    // Record the values of the current day.
    let now = Local::now();
    let entry = format!("/home/longlb/Documents/journal/{}", now.format("%Y-%m-%d"));

    // let (year, month, day) = (now.year(), now.month(), now.day());

    // Taking the argument following the journal command.
    // let args: Vec<String> = env::args().collect();
    // if args.len() == 1 {
    //     println!("present day.");
    // } else {
    //     match args[1].as_str() {
    //         "today" => println!("today {:?} {} {}", year, month, day),
    //         "tomorrow" => println!("tmr {}", now),
    //         "yesterday" => println!("ystdy {}", now),
    //         _ => println!("nuthin"),
    //     }
    // }

    Command::new("mousepad")
        .arg(entry)
        .spawn()
        .expect("Cannot open journal");
}
