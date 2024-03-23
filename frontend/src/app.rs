use crate::{comms, sample_service::EchoRequest, util::FailStatus};
use eframe::egui::{self, Frame, Margin, Vec2};
use std::{
    process::Termination,
    sync::mpsc::{channel, Receiver, Sender},
};

#[derive(Debug)]
pub struct Frontend {
    cycle: u32,
    show_busy: bool,
    show_error: bool,
    message: Option<String>,
    error: Option<FailStatus>,
    send: Sender<Result<String, FailStatus>>,
    recv: Receiver<Result<String, FailStatus>>,
}

impl Frontend {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let (tx, rx) = channel();

        Self {
            cycle: 0,
            show_busy: false,
            show_error: false,
            message: None,
            error: None,
            send: tx,
            recv: rx,
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
}

impl eframe::App for Frontend {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel")
            .resizable(false)
            .frame(Frame {
                inner_margin: Margin::symmetric(6., 10.),
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing = Vec2 { x: 6., y: 6. };
                ui.spacing_mut().button_padding = Vec2 { x: 4., y: 4. };
                ui.horizontal(|ui| {
                    ui.menu_button("Menu", Self::sample_menu);
                    let _ = ui.button("Another button");
                });

                ui.separator();

                ui.heading("WIP");
            });

        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(true)
            .min_height(128.)
            .frame(Frame {
                inner_margin: Margin::symmetric(6., 10.),
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing = Vec2 { x: 6., y: 6. };
                ui.spacing_mut().button_padding = Vec2 { x: 4., y: 4. };
                ui.horizontal(|ui| {
                    if ui.button("Request stuff").clicked() {
                        self.cycle += 2;
                        self.show_busy = true;
                        let payload = format!("{}", self.cycle.clone());
                        let send = self.send.clone();

                        tokio::spawn(async move {
                            send.send(comms::echo(EchoRequest { payload }).await)
                                .report();
                        });
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

                if let Ok(msg) = self.recv.try_recv() {
                    match msg {
                        Ok(response) => self.message = Some(response),
                        Err(error) => {
                            self.error = Some(error);
                            self.show_error = true;
                        }
                    };

                    self.show_busy = false;
                } else if self.show_busy {
                    ui.horizontal(|ui| {
                        ui.label("Processing...");
                        ui.add(egui::widgets::Spinner::new());
                    });
                }

                let mut show = self.show_error;
                if self.show_error {
                    egui::Window::new("Error")
                        .open(&mut self.show_error)
                        .resizable(false)
                        .collapsible(false)
                        .show(ctx, |ui| {
                            ui.label(self.error.as_ref().unwrap().to_string());
                            if ui.button("OK").clicked() {
                                show = false;
                            }
                        });
                    if !show {
                        self.show_error = false;
                        self.error = None;
                    }
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Item 01");
                ui.heading("Item 02");
            });
        });
    }
}
