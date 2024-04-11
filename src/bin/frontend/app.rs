use crate::{
    comms::{self, PipeMsg},
    ipc_srv::EchoRequest,
};
use eframe::egui;
use sandbox_research::FailStatus;
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use tokio::runtime::Runtime;

pub struct Frontend {
    rt: Runtime,
    cycle: u32,
    show_error: bool,
    show_settings: bool,
    message: Option<String>,
    error: Option<FailStatus>,
    comm_tx: Sender<PipeMsg>,
    comm_rx: Receiver<PipeMsg>,
}

impl Frontend {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        let (comm_tx, comm_rx) = channel();

        Self {
            rt,
            cycle: 0,
            show_error: false,
            show_settings: false,
            message: None,
            error: None,
            comm_tx,
            comm_rx,
        }
    }

    fn list_menu(ui: &mut egui::Ui) {
        let _ = ui.button("Item 01");
        let _ = ui.button("Item 02");

        // ui.menu_button("Sub Menu", |ui| {
        //     let _ = ui.button("Item 01");
        //     let _ = ui.button("Item 02");
        // });

        if ui.button("Close").clicked() {
            ui.close_menu();
        }
    }

    fn settings_dialogue(&mut self, ctx: &egui::Context) {
        let mut toggle = self.show_settings;

        egui::Window::new("Settings")
            .open(&mut toggle)
            .resizable(false)
            .collapsible(false)
            .show(ctx, |ui| {
                ui.label("You should see the settings panel here");
                if ui.button("OK").clicked() {
                    self.show_settings = false;
                }
            });
    }

    fn error_dialogue(&mut self, ctx: &egui::Context) {
        let mut toggle = self.show_error;

        egui::Window::new("Error")
            .open(&mut toggle)
            .resizable(false)
            .collapsible(false)
            .show(ctx, |ui| {
                ui.label(self.error.as_ref().unwrap().to_string());
                if ui.button("OK").clicked() {
                    self.show_error = false;
                    self.error = None;
                }
            });
    }
}

impl eframe::App for Frontend {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        {
            match self.comm_rx.try_recv() {
                Ok(PipeMsg::Echo(data)) => {
                    self.message = Some(data);
                }
                Ok(PipeMsg::Fail(error)) => {
                    self.error = Some(error);
                    self.show_error = true;
                }
                Err(error) => match error {
                    TryRecvError::Empty => (),
                    TryRecvError::Disconnected => {
                        tracing::error!("[!] comm_tx has been disconnected")
                    }
                },
            }
        }

        egui::TopBottomPanel::top("top_panel")
            .resizable(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.menu_button("File", Self::list_menu);
                    if ui.button("Settings").clicked() {
                        self.show_settings = true;
                    };
                });
            });

        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .min_height(128.)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Request stuff").clicked() {
                        self.cycle += 2;

                        comms::echo(
                            &self.rt,
                            self.comm_tx.clone(),
                            EchoRequest {
                                payload: self.cycle.clone().to_string(),
                            },
                        );
                    };

                    if ui.button("Clear stuff").clicked() {
                        self.cycle = 0;
                        self.message = None;
                    }

                    ui.label(format!("Count: {}", &self.cycle))
                });

                if let Some(message) = &self.message {
                    ui.label(message);
                } else {
                    ui.label("No messages yet");
                }

                self.error_dialogue(ctx);
                self.settings_dialogue(ctx);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Item 01");
                ui.heading("Item 02");
            });
        });
    }
}
