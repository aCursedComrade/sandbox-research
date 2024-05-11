mod client;
mod comms;
mod state;
mod widgets;

use client::Frontend;
use eframe::egui::vec2;

fn main() -> eframe::Result<()> {
    tracing_subscriber::fmt::init();

    let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
    let _guard = rt.enter();

    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder {
            min_inner_size: Some(vec2(1024., 512.)),
            inner_size: Some(vec2(1280., 768.)),
            ..Default::default()
        },
        follow_system_theme: false,
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        sandbox_research::APP_NAME,
        options,
        Box::new(|cc| Box::new(Frontend::new(cc))),
    )
}
