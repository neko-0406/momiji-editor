use eframe::egui::{self, Visuals};
use momiji_editor::{MomijiEditor, setup_custom_fonts};

fn main() {
    // windowの設定
    let options = eframe::NativeOptions {
        viewport: egui::viewport::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    // アプリの実行
    let _ = eframe::run_native(
        "momiji-editor",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(Visuals::light());
            setup_custom_fonts(&cc.egui_ctx);
            Ok(Box::new(MomijiEditor::default()))
        }),
    );
}
