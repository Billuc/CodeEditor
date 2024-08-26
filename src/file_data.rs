use std::{fs, path::PathBuf};

pub struct FileData {
    language: String,
    content: String,
    name: String,
    path: String,
}

impl FileData {
    pub fn new(path_str: String) -> Self {
        let path = PathBuf::from(path_str.clone());

        let name = if let Some(name) = path.file_name() {
            name.to_string_lossy().to_string()
        } else {
            "".to_string()
        };

        let language = if let Some(ext) = path.extension() {
            ext.to_string_lossy().to_string()
        } else {
            "".to_string()
        };

        let content = fs::read_to_string(path).expect("Path does not exist or file doesn't contain UTF8 data");

        FileData {
            content,
            language,
            name,
            path: path_str.clone()
        }
    }

    pub fn from_path(path: PathBuf) -> Self {
        FileData::new(path.to_string_lossy().to_string())
    }

    pub fn get_path(self: &Self) -> &str {
        &self.path
    }

    pub fn get_name(self: &Self) -> &str {
        &self.name
    }

    pub fn get_language(self: &Self) -> &str {
        &self.language
    }

    pub fn borrow_content(self: &mut Self) -> &mut String {
        &mut self.content
    }
}