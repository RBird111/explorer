use serde::Serialize;
use std::{fs::DirEntry, path::Path};

#[derive(Debug, Clone, Serialize)]
pub struct Dir {
    pub dir: String,
    pub files: Vec<Entry>,
}

impl Dir {
    pub fn new() -> Self {
        // let directory = std::env::current_dir().expect("Can't find current directory");
        let home = std::env::var("HOME").expect("Error reading $HOME");
        println!("Home => {home}");
        let directory = Path::new(&home);

        let dir = directory.to_string_lossy().to_string();
        let mut files = vec![];

        for entry_result in directory
            .read_dir()
            .expect("Can't read contents of directory")
        {
            let entry = entry_result.expect("Error reading entry");
            files.push(Entry::new(entry));
        }

        files.sort_unstable_by(|a, b| a.file_type.cmp(&b.file_type));

        Self { dir, files }
    }

    pub fn update(&self, file: &str) -> Self {
        let file = Path::new(file);
        if file.is_dir() {
            let directory = std::env::current_dir()
                .expect("Can't find current directory")
                .join(file);

            let dir = directory.to_string_lossy().to_string();
            let mut files = vec![];

            for entry_result in directory
                .read_dir()
                .expect("Can't read contents of directory")
            {
                let entry = entry_result.expect("Error reading entry");
                files.push(Entry::new(entry))
            }

            files.sort_unstable_by(|a, b| a.file_type.cmp(&b.file_type));

            return Self { dir, files };
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
