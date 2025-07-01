use eframe::egui;
use momiji_editor::{MomijiEditor, setup_custom_fonts};

fn main() {
    // windowの設定
    let options = eframe::NativeOptions {
        viewport: egui::viewport::ViewportBuilder::default()
            .with_inner_size([600.0, 800.0]),
        ..Default::default()
    };

    // アプリの実行
    let _ = eframe::run_native(
        "momiji-editor",
        options,
        Box::new(|cc| {
            setup_custom_fonts(&cc.egui_ctx);
            Ok(Box::new(MomijiEditor::default()))
        }),
    );
}
