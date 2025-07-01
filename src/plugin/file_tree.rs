use std::path::{Path, PathBuf};

pub struct FileItem {
    is_file: bool,
    file_name: String,
    file_path: String,
    children: Option<Vec<Self>>
}

impl FileItem {
    pub fn new_file(name: &str, abs_dir_path: &Path) -> Self {
        let file_path = abs_dir_path.join(name);
        return Self {
            is_file: file_path.is_file(),
            file_name: name.to_owned(),
            file_path: file_path.to_string_lossy().into_owned(),
            children: None
        };
    }

    pub fn new_dir(name: &str, abs_dir_path: &Path) -> Self {
        let dir_path: PathBuf = abs_dir_path.join(name);
        let children: Option<Vec<Self>> = {
            let entries = dir_path.read_dir().expect("フォルダの読み込みに失敗しました");
            
        };
    }
}