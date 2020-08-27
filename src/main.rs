use journaling::run;

fn main() {
    match run() {
        Ok(_) => println!("Opening journal"),
        Err(e) => println!("Journal failed to open. Error: {}", e),
    }
}
