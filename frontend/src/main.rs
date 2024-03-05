use eframe::egui;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Sample Frontend",
        native_options,
        Box::new(|cc| Box::new(Frontend::new(cc))),
    )
}

#[derive(Default)]
struct Frontend {}

impl Frontend {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for Frontend {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello there!");
        });
    }
}
