use eframe::egui;
use sandbox_research::Status;

/// Application settings dialogue
pub fn settings_dialogue(ctx: &egui::Context, visible: &mut bool) {
    egui::Window::new("Settings")
        .open(visible)
        .resizable(false)
        .collapsible(false)
        .show(ctx, |ui| {
            ui.label("You should see the settings panel here");
        });
}

/// Application error dialogue
pub fn error_dialogue(ctx: &egui::Context, visible: &mut bool, error: Option<Status>) {
    egui::Window::new("Error")
        .open(visible)
        .resizable(false)
        .collapsible(false)
        .show(ctx, |ui| {
            ui.label(error.unwrap_or(Status::EmptyError).to_string());
        });
}
