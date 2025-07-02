use std::path::Path;

use eframe::egui::{ComboBox, Ui};

use crate::plugin::file_tree::FileItem;

pub fn display_tree(ui: &mut Ui, root_dir: Option<&str>) {
    match root_dir {
        Some(root_dir) => {
            let path = Path::new(root_dir);
            let root_item = FileItem::new(path);
            ui.add(ComboBox::)
        }
        None => {}
    }
}