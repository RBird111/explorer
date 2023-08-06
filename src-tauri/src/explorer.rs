use serde::Serialize;
use std::{fs::DirEntry, path::Path};
use tauri::api::path;

#[derive(Debug, Clone, Serialize)]
pub struct Dir {
    pub dir: String,
    pub files: Vec<Entry>,
}

impl Dir {
    /// Creates a new `Dir` instance at `$HOME`
    pub fn new() -> Self {
        let mut new_dir = Self {
            dir: String::new(),
            files: vec![],
        };

        let home = path::home_dir().expect("Error trying to find HOME directory");
        let directory = Path::new(&home);

        new_dir.sort_files(directory);

        new_dir
    }

    /// Moves `Dir` up one level (if not already at root)
    pub fn go_up(&mut self) -> Self {
        let cur_path = &self.dir.clone();
        let cur_dir = Path::new(cur_path);

        if let Some(directory) = cur_dir.ancestors().skip(1).next() {
            self.sort_files(directory);
            self.clone()
        } else {
            self.clone()
        }
    }

    /// Moves `Dir` down one level to `file` (if it's a directory)
    pub fn go_down_to(&mut self, file: &str) -> Self {
        let file = Path::new(file);
        let directory = Path::new(&self.dir).join(file);

        if directory.is_dir() {
            self.sort_files(&directory);

            self.clone()
        } else {
            self.clone()
        }
    }

    /// Sorts directory entries by `Entry.file_type` (directory or file)
    /// and then by name. Symlinks are ignored for now.
    ///
    /// TODO: symlinks
    fn sort_files(&mut self, directory: &Path) {
        let dir = directory.to_string_lossy().to_string();
        let (mut files, mut dirs) = (vec![], vec![]);

        for entry_result in directory
            .read_dir()
            .expect("Can't read contents of directory")
        {
            let entry = entry_result.expect("Error reading entry");
            let file_type = entry.file_type().expect("Error reading file type");

            if file_type.is_file() {
                files.push(Entry::new(entry))
            } else if file_type.is_dir() {
                dirs.push(Entry::new(entry))
            }
        }
        dirs.sort_unstable_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        files.sort_unstable_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

        self.dir = dir;
        self.files = [dirs, files].concat();
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
