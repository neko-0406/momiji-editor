mod plugin;
use eframe::egui::{self, FontData, FontDefinitions};

#[derive(Default)]
pub struct MomijiEditor;

impl MomijiEditor {
    
}

impl eframe::App for MomijiEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_menu_bar").show(ctx, |ui| {
            ui.label("メニューバーになる予定");
        });

        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.label("ステータスバーになる予定");
        });

        egui::SidePanel::left("side_menu_bar").show(ctx, |_ui| {

        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("なんか置く場所")
        });
    }
}

// フォントセットアップ用の関数
pub fn setup_custom_fonts(ctx: &egui::Context) {
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