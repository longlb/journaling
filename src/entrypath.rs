use super::execute;
use std::io;
use std::path::PathBuf;

pub struct EntryPath {
    path: PathBuf,
}

impl EntryPath {
    pub fn new() -> Self {
        Self {
            // It's possible to use dirs crate for home directory variable rather than hard code.
            path: PathBuf::from("/home/longlb/Documents/journal"),
        }
    }

    pub fn add_dir(&mut self, dir: String) -> io::Result<()> {
        self.path.push(dir);
        if !self.path.is_dir() {
            execute("mkdir", self.to_str()?)?;
        }
        Ok(())
    }

    pub fn add_file(&mut self, file: String) -> io::Result<()> {
        self.path.push(file);
        if !self.path.is_file() {
            execute("touch", self.to_str()?)?;
        }
        Ok(())
    }

    pub fn to_str(&self) -> io::Result<&str> {
        self.path.to_str().ok_or(io::Error::new(
            io::ErrorKind::Other,
            "Path has invalid unicode",
        ))
    }
}

// #[cfg()]
