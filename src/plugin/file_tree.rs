use std::{fs::read_dir, path::Path};

pub struct FileItem {
    is_file: bool,
    file_name: String,
    file_path: String,
    children: Option<Vec<Self>>
}

impl FileItem {
    pub fn new(file_path: &Path) -> Self {
        let file_name = file_path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("")
            .to_string();
        
        let is_file = file_path.is_file();
        
        if is_file {
            Self {
                is_file,
                file_name,
                file_path: file_path.to_string_lossy().to_string(),
                children: None,
            }
        } else {
            let children = read_dir(file_path)
                .ok()
                .and_then(|entries| {
                    let mut items = Vec::new();
                    for entry in entries {
                        if let Ok(entry) = entry {
                            items.push(Self::new(&entry.path()));
                        }
                    }
                    if items.is_empty() {
                        None
                    } else {
                        Some(items)
                    }
                });
            Self {
                is_file,
                file_name,
                file_path: file_path.to_string_lossy().to_string(),
                children,
            }
        }
    }
}