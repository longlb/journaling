use std::process::Command;
use std::{env, io};

mod date;
mod entrypath;
use entrypath::EntryPath;

// 1. Prepare for the day ahead.
// "The  wise  will  start  each  day with the thought, ‘Fortune gives us nothing which we can really own.’"
// 2. Put the day up for review.
// "Did I follow my plans for the day? Was I prepared enough? What could I do better? What have I learned that will help me tomorrow?"
// - Seneca on journaling.

// Main function to open journal entries.
pub fn run() -> io::Result<()> {
    // Create an Entry path based on whether there's an argument or not.
    // If the argument isn't matched, default to the current day.
    // Ignore all arguments after the first.
    let args: Vec<String> = env::args().collect();
    let entrypath = match args.len() == 1 {
        true => EntryPath::today(),
        false => EntryPath::input(&args[1]),
    };

    // Open a text editor to the path with the final entry date.
    // In my case, I use mousepad to journal so I open that.
    // Switched to gnome, so I gotta use gedit now.
    execute("gedit", entrypath.as_str()?)
}

// Execute a linux command with a single argument.
fn execute(command: &str, arg: &str) -> io::Result<()> {
    Command::new(command).arg(arg).spawn()?;
    Ok(())
}
