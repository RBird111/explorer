use serde::Serialize;
use std::{fs::DirEntry, path::Path};

#[derive(Debug, Clone, Serialize)]
pub struct Dir {
    pub dir: String,
    pub files: Vec<Entry>,
}

impl Dir {
    pub fn new() -> Self {
        let home = std::env::var("HOME").expect("Error reading $HOME");
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

    pub fn go_up(&mut self) -> Self {
        let cur_dir = Path::new(&self.dir);
        let directory = cur_dir.ancestors().skip(1).next().unwrap();

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

        self.dir = dir;
        self.files = files;
        self.clone()
    }

    pub fn go_down_to(&mut self, file: &str) -> Self {
        let file = Path::new(file);
        let directory = Path::new(&self.dir).join(file);

        if directory.is_dir() {
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

            self.dir = dir;
            self.files = files;
            return self.clone();
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
