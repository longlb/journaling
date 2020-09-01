use super::date::SuperDate;
use super::execute;
use rand::seq::IteratorRandom;
use regex::Regex;
use std::io;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

// File extensions.
enum Ext {
    Dir(String),
    File(String),
}

impl Ext {
    fn contents(&self) -> &str {
        match self {
            Ext::Dir(s) => &s,
            Ext::File(s) => &s,
        }
    }
}

// The main journal entry path struct.
pub struct EntryPath {
    path: PathBuf,
}

impl EntryPath {
    // Generate an EntryPath with the home path, no date.
    fn home() -> Self {
        Self {
            // It's possible to use dirs crate for home directory variable rather than hard code.
            // But I'll leave it like this for now, since I'm the only one using it.
            path: PathBuf::from("/home/longlb/Documents/journal"),
        }
    }

    // Generate an EntryPath for today.
    pub fn today() -> Self {
        EntryPath::home().attach_date(SuperDate::today())
    }

    // Generate an EntryPath based on user input.
    pub fn input(input: &str) -> Self {
        let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
        match input {
            _ if re.is_match(input) => EntryPath::home().attach_date(SuperDate::process(input)),
            "yesterday" => EntryPath::home().attach_date(SuperDate::days(-1)),
            "tomorrow" => EntryPath::home().attach_date(SuperDate::days(1)),
            "random" => EntryPath::random(),
            _ => EntryPath::today(),
        }
    }

    // Return a str of the entry's path.
    pub fn as_str(&self) -> io::Result<&str> {
        self.path
            .to_str()
            .ok_or(Error::new(ErrorKind::Other, "Path has invalid unicode"))
    }

    fn attach_date(mut self, date: SuperDate) -> Self {
        self.attach_ext(Ext::Dir(date.year()))
            .expect("Directory can't be made");
        self.attach_ext(Ext::Dir(date.month()))
            .expect("Directory can't be made");
        self.attach_ext(Ext::File(date.day()))
            .expect("File can't be made");
        self
    }

    fn attach_ext(&mut self, ext: Ext) -> io::Result<()> {
        self.path.push(ext.contents());
        if !self.path.exists() {
            println!("not valid {}", self.path.display());
            match ext {
                Ext::Dir(_) => execute("mkdir", self.as_str()?)?,
                Ext::File(_) => execute("touch", self.as_str()?)?,
            }
        }
        Ok(())
    }

    fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            path: PathBuf::from("/home/longlb/Documents/journal")
                .read_dir()
                .expect("read_dir call failed")
                .choose(&mut rng)
                .unwrap()
                .unwrap()
                .path()
                .read_dir()
                .expect("read_dir call failed")
                .choose(&mut rng)
                .unwrap()
                .unwrap()
                .path()
                .read_dir()
                .expect("read_dir call failed")
                .choose(&mut rng)
                .unwrap()
                .unwrap()
                .path(),
        }
    }
}
