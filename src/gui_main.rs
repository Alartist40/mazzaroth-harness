use eframe::egui;
use mazzaroth::{MazzarothEngine, MazzarothVisualizerApp};
use std::path::PathBuf;

fn main() -> eframe::Result<()> {
    let db_path = PathBuf::from("data/mazzaroth.db");
    if let Some(parent) = db_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    let engine = MazzarothEngine::open(&db_path).unwrap_or_else(|_| MazzarothEngine::in_memory().unwrap());

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("🌌 Mazzaroth — Celestial Cognitive Galaxy"),
        ..Default::default()
    };

    eframe::run_native(
        "Mazzaroth Galaxy",
        native_options,
        Box::new(|_cc| Ok(Box::new(MazzarothVisualizerApp::new(engine)))),
    )
}
