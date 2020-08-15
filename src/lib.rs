use std::process::Command;
use std::{env, io};

mod entryday;
mod entrypath;

pub fn run() -> io::Result<()> {
    // Initialize the general journal path and the entry date based on user input..
    let mut path = entrypath::EntryPath::new();
    let mut day = entryday::EntryDay::new();

    // Get the argument if there is any, then apply it to the date.
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        day.input(&args[1]);
    }

    // Enter the day's year and month directories.
    path.add_dir(day.year())?;
    path.add_dir(day.month())?;

    // Add the day to the path.
    path.add_file(day.day())?;

    // Execute the combined command.
    execute("mousepad", path.to_str()?)?;
    Ok(())
}

// Execute a linux command with a single argument.
fn execute(command: &str, arg: &str) -> io::Result<()> {
    Command::new(command).arg(arg).spawn()?;
    Ok(())
}
