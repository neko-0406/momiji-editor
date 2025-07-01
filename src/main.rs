use eframe::egui::{self, FontData, FontDefinitions};

#[derive(Default)]
struct MomijiEditor;

impl eframe::App for MomijiEditor {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        
    }
}

fn setup_custom_fonts(ctx: &egui::Context) {
    // フォント設定の取得
    let mut fonts = FontDefinitions::default();

    // 日本語フォント(可変ウェイト)を追加
    fonts.font_data.insert(
        "noto_sans_jp".to_owned(),
        FontData::from_static(include_bytes!("../assets/NotoSansJP.ttf")).into()
    );

    // フォントファミリーに追加
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "noto_sans_jp".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("noto_sans_jp".to_owned());

    ctx.set_fonts(fonts);
}

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
