use eframe::egui;

#[derive(Default)]
struct MomijiEditor;

impl eframe::App for MomijiEditor {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        
    }
}

fn main() {
    // windowの設定
    let options = eframe::NativeOptions {
        viewport: egui::viewport::ViewportBuilder::default()
            .with_inner_size([600.0, 800.0]),
        ..Default::default()
    };

    // アプリの実行
    eframe::run_native(
        "momiji-editor",
        options,
        Box::new(|_cc| {
            Ok(Box::new(MomijiEditor::default()))
        }),
    );
}
