#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

use sample_service::exchange_client::ExchangeClient;
use sample_service::EchoRequest;

pub(crate) mod sample_service {
    tonic::include_proto!("ipc_interface");
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_min_inner_size([1024., 512.])
            .with_inner_size([1280., 768.]),
        follow_system_theme: false,
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "Sample Frontend",
        options,
        Box::new(|cc| Box::new(Frontend::new(cc))),
    )
}

#[derive(Debug)]
struct Frontend {
    cycle: u32,
    message: Option<String>,
    rt: tokio::runtime::Runtime,
}

impl Frontend {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        Self {
            cycle: 0,
            message: None,
            rt: runtime,
        }
    }

    fn sample_menu(ui: &mut egui::Ui) {
        let _ = ui.button("Item 01");
        let _ = ui.button("Item 02");

        ui.menu_button("Sub Menu", |ui| {
            let _ = ui.button("Item 01");
            let _ = ui.button("Item 02");
        });

        if ui.button("Close").clicked() {
            ui.close_menu();
        }
    }

    async fn call_server(payload: String) -> Result<String, Box<dyn std::error::Error>> {
        let mut client = ExchangeClient::connect("http://[::1]:50055").await?;

        let request = tonic::Request::new(EchoRequest { payload });

        let response = client.echo(request).await?;

        Ok(response.into_inner().message)
    }

    fn get_messages(&mut self) {
        let payload = format!("{}", &self.cycle.clone());

        let demo = self.rt.spawn(async {
            match Self::call_server(payload).await {
                Ok(msg) => msg,
                Err(_) => String::from("[!] Nope (error talking to server)"),
            }
        });

        self.message = Some(self.rt.block_on(demo).unwrap());
        self.cycle += 2
    }
}

impl eframe::App for Frontend {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel")
            .resizable(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.menu_button("Menu", Self::sample_menu);
                    let _ = ui.button("Another button");
                });

                ui.separator();

                ui.heading("Top Panel");
            });

        egui::SidePanel::left("left_panel")
            .resizable(true)
            .min_width(128.)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    let _ = ui.button("Item 01");
                    let _ = ui.button("Item 02");
                })
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello there!");

            ui.horizontal(|ui| {
                if ui.button("Request stuff").clicked() {
                    self.get_messages();
                };

                if ui.button("Clear stuff").clicked() {
                    self.cycle = 0;
                    self.message = None;
                }
            });

            if let Some(message) = &self.message {
                ui.label(message);
            }
        });
    }
}
