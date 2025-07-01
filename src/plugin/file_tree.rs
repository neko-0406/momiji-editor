use eframe::epaint::tessellator::Path;

pub struct FileItem {
    is_file: bool,
    file_name: String,
    file_path: String,
    children: Option<Vec<Self>>
}

// impl FileItem {
//     pub fn new_file(name: &str, abs_dir_path: &Path) -> Self {
//         let file_name: &str = name;
//         let file_path = {};
//     }
// }