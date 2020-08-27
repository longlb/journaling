use std::process::Command;
use std::{env, io};

mod entry;
use entry::Entry;

// 1. Prepare for the day ahead.
// "The  wise  will  start  each  day with the thought, ‘Fortune gives us nothing which we can really own.’"
// 2. Put the day up for review.
// "Did I follow my plans for the day? Was I prepared enough? What could I do better? What have I learned that will help me tomorrow?"
// - Seneca on journaling.

pub fn run() -> io::Result<()> {
    // Initialize the general journal path and the entry date.
    let mut entry = Entry::new();

    // Change the entry date according to the argument if one is given.
    // If the argument isn't matched, default to the current day.
    // Ignore everything after the first argument.
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        entry.input(&args[1]);
    }

    // Attach the entry date's items to the path.
    // I wanted to make this auto, but I fucked up pretty royally.
    // I'll attempt it again post refactor.
    entry.add_dir(entry.year())?;
    entry.add_dir(entry.month())?;
    entry.add_file(entry.day())?;

    // Open a text editor to the path with the final entry date.
    // In my case, my preferred text editor for this is mousepad.
    execute("mousepad", entry.to_str()?)
}

// Execute a linux command with a single argument.
fn execute(command: &str, arg: &str) -> io::Result<()> {
    Command::new(command).arg(arg).spawn()?;
    Ok(())
}
