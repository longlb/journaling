use super::execute;
use std::io::{Error, ErrorKind, Result};
use std::path::PathBuf;

pub struct EntryPath {
    path: PathBuf,
}

impl EntryPath {
    pub fn new() -> Self {
        Self {
            path: PathBuf::from("/home/longlb/Documents/journal"),
        }
    }

    pub fn add_dir(&mut self, dir: String) -> Result<()> {
        self.path.push(dir);
        if !self.path.is_dir() {
            execute("mkdir", self.to_str()?)?;
        }
        Ok(())
    }

    pub fn add_file(&mut self, file: String) -> Result<()> {
        self.path.push(file);
        if !self.path.is_file() {
            execute("touch", self.to_str()?)?;
        }
        Ok(())
    }

    pub fn to_str(&self) -> Result<&str> {
        self.path
            .to_str()
            .ok_or(Error::new(ErrorKind::Other, "Path has invalid unicode"))
    }
}
