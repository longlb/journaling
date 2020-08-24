use std::process::Command;
use std::{env, io};

mod entryday;
mod entrypath;

// 1. Prepare for the day ahead.
// "The  wise  will  start  each  day with the thought, ‘Fortune gives us nothing which we  can  really  own.’"
// 2. Put the day up for review.
// "Did I follow my plans for the day? Was I prepared enough? What could I do better? What have I learned that will help me tomorrow?"
// - Seneca on journaling.

pub fn run() -> io::Result<()> {
    // Initialize the general journal path and the entry date.
    let mut path = entrypath::EntryPath::new();
    let mut day = entryday::EntryDay::new();

    // Apply the given argument to the date if there is one.
    // If the argument isn't matched, open the current day's entry.
    // Ignore
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        day.input(&args[1]);
    }

    // Attach the entry's year and month to the base path.
    // If either don't exist yet, they will be made.
    path.add_dir(day.year())?;
    path.add_dir(day.month())?;

    // Add the entry's day to the path.
    // If it doesn't exist, a file with it's name will be made.
    path.add_file(day.day())?;

    // Execute the command with the finalized path.
    execute("mousepad", path.to_str()?)?;
    Ok(())
}

// Execute a linux command with a single argument.
fn execute(command: &str, arg: &str) -> io::Result<()> {
    Command::new(command).arg(arg).spawn()?;
    Ok(())
}
