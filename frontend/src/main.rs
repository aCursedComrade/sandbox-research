use std::time::Duration;

fn main() -> eframe::Result<()> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let _guard = rt.enter();

    std::thread::spawn(move || {
        rt.block_on(async {
            loop {
                tokio::time::sleep(Duration::from_secs(3600)).await;
            }
        })
    });

    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_min_inner_size([1024., 512.])
            .with_inner_size([1280., 768.]),
        follow_system_theme: false,
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "Sample Frontend",
        options,
        Box::new(|cc| Box::new(frontend::Frontend::new(cc))),
    )
}
