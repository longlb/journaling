use journaling::run;

fn main() {
    match run() {
        Ok(_) => println!("Opening journal"),
        Err(_) => println!("Journal failed to open"),
    }
}
