use std::{fs::DirEntry, path::Path};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Dir {
    pub directory: String,
    pub files: Vec<Entry>,
}

impl Dir {
    pub fn new() -> Self {
        let dir = std::env::current_dir().expect("Can't find current directory");

        let directory = dir.to_string_lossy().to_string();
        let mut files = vec![];

        for entry_result in dir.read_dir().expect("Can't read contents of directory") {
            let entry = entry_result.expect("Error reading entry");
            files.push(Entry::new(entry))
        }

        Self { directory, files }
    }

    pub fn update(&self, file: &str) -> Self {
        let file = Path::new(file);
        if file.is_dir() {
            let dir = std::env::current_dir()
                .expect("Can't find current directory")
                .join(file);

            let directory = dir.to_string_lossy().to_string();
            let mut files = vec![];

            for entry_result in dir.read_dir().expect("Can't read contents of directory") {
                let entry = entry_result.expect("Error reading entry");
                files.push(Entry::new(entry))
            }

            return Self { directory, files };
        }

        self.clone()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Entry {
    pub name: String,
    pub file_type: String,
}

impl Entry {
    fn new(entry: DirEntry) -> Self {
        let name = entry.file_name().to_string_lossy().to_string();
        let f_type = entry.file_type().expect("Can't read file type");

        let file_type = if f_type.is_dir() {
            "directory".to_string()
        } else if f_type.is_file() {
            "file".to_string()
        } else {
            "symlink".to_string()
        };

        Self { name, file_type }
    }
}
